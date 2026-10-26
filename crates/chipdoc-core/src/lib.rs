pub mod cleaner;
pub mod emitter;
pub mod error;
pub mod figures;
pub mod model;
pub mod pdf_loader;
pub mod query;
pub mod registers;
pub mod sanity;
pub mod stitcher;
pub mod tables;
pub mod xref;

use crate::cleaner::{assemble_text_lines, clean_pages};
use crate::emitter::{build_section_files, ensure_claudeignore_entry, write_agent_bundle};
use crate::error::Result;
use crate::figures::detect_page_figures;
use crate::model::{ConversionBundle, ConvertConfig, DocMetadata, PageBlock, TocEntry};
use crate::pdf_loader::{
    bind_pdfium, compute_pdf_fingerprint, load_pdf_document, render_detected_figures,
};
use crate::query::load_bundle_status;
use crate::registers::suppress_visual_register_bitboxes;
use crate::sanity::{compute_sanity_report, SanityInputs};
use crate::stitcher::{assemble_page_blocks, compute_section_shards, stitch_multipage_tables};
use crate::tables::extract_page_tables;
use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;

/// Summary returned by `inspect_pdf`.
#[derive(Debug, Clone)]
pub struct PdfInspection {
    pub metadata: DocMetadata,
    pub toc: Vec<TocEntry>,
    pub total_spans: usize,
    pub total_grid_segments: usize,
    pub total_graphics: usize,
    pub total_images: usize,
}

/// Inspects a PDF file and returns its metadata, bookmark tree, and object statistics.
pub fn inspect_pdf(
    input_pdf: &Path,
    page_filter: Option<&[usize]>,
    pdfium_lib_dir: Option<&Path>,
) -> Result<PdfInspection> {
    let pdfium = bind_pdfium(pdfium_lib_dir)?;
    let (_doc, metadata, toc, raw_pages) = load_pdf_document(&pdfium, input_pdf, page_filter)?;

    let total_spans = raw_pages.iter().map(|p| p.spans.len()).sum();
    let total_grid_segments = raw_pages.iter().map(|p| p.grid_segments.len()).sum();
    let total_graphics = raw_pages.iter().map(|p| p.graphic_bboxes.len()).sum();
    let total_images = raw_pages.iter().map(|p| p.image_bboxes.len()).sum();

    Ok(PdfInspection {
        metadata,
        toc,
        total_spans,
        total_grid_segments,
        total_graphics,
        total_images,
    })
}

/// Executes the full `chipdoc` conversion pipeline on a PDF and writes an Agent Documentation Bundle.
/// When `!config.force` and `config.page_filter.is_none()`, returns an existing up-to-date bundle
/// in <2ms if the source PDF fingerprint matches `meta.json`.
pub fn convert_pdf(config: &ConvertConfig) -> Result<ConversionBundle> {
    let start_ts = Instant::now();

    if !config.force && config.page_filter.is_none() {
        if let Some(stem) = config.input_pdf.file_stem().and_then(|s| s.to_str()) {
            let candidate_dir = config.output_root.join(stem);
            let index_md_path = candidate_dir.join("INDEX.md");
            let xref_tsv_path = candidate_dir.join("index").join("xref.tsv");
            if index_md_path.exists() && xref_tsv_path.exists() {
                if let (Ok((hash, _)), Some(mut status)) = (
                    compute_pdf_fingerprint(&config.input_pdf),
                    load_bundle_status(&candidate_dir),
                ) {
                    if !status.metadata.source_hash.is_empty()
                        && status.metadata.source_hash == hash
                    {
                        if config.update_claudeignore {
                            let _ = ensure_claudeignore_entry(&config.output_root);
                        }
                        status.sanity.cache_hit = true;
                        status.sanity.elapsed_ms = start_ts.elapsed().as_millis();
                        let single_md_path = config
                            .single_file
                            .then(|| candidate_dir.join(format!("{stem}.md")))
                            .filter(|p| p.exists());
                        return Ok(ConversionBundle {
                            bundle_dir: candidate_dir,
                            index_md_path,
                            single_md_path,
                            metadata: status.metadata,
                            sanity: status.sanity,
                        });
                    }
                }
            }
        }
    }

    let pdfium = bind_pdfium(config.pdfium_lib_dir.as_deref())?;

    // 1. Load PDF metadata, outline tree, and raw page primitives
    let (doc, metadata, toc, mut raw_pages) =
        load_pdf_document(&pdfium, &config.input_pdf, config.page_filter.as_deref())?;

    // 2. Clean running headers/footers, boilerplate, and normalize Unicode/subscripts
    let clean_stats = clean_pages(&mut raw_pages);

    // 3. Process each page in parallel with Rayon:
    //    a) Reconstruct geometric tables
    //    b) Deduplicate visual 16-/32-bit register bit-boxes
    //    c) Detect vector/raster figures and suppress scrambled diagram labels
    let per_page_extracted: Vec<_> = raw_pages
        .par_iter_mut()
        .map(|page| {
            let tables = extract_page_tables(page);
            let lines_after_tables = assemble_text_lines(&page.spans, 2.5);
            let bitboxes_removed =
                suppress_visual_register_bitboxes(&mut page.spans, &lines_after_tables, &tables);
            let (figures, fig_stats) = detect_page_figures(page, &tables);
            (tables, figures, bitboxes_removed, fig_stats)
        })
        .collect();

    // 4. Assemble ordered page blocks and collect figure regions for rendering
    let mut all_blocks: Vec<PageBlock> = Vec::new();
    let mut all_figures = Vec::new();
    let mut visual_bitboxes_deduplicated = 0usize;
    let mut figures_detected = 0usize;
    let mut figure_text_spans_suppressed = 0usize;
    let mut current_peripheral = String::from("SYS");

    for (page, (tables, figures, bitboxes, fig_stats)) in raw_pages.iter().zip(per_page_extracted) {
        visual_bitboxes_deduplicated += bitboxes;
        figures_detected += fig_stats.figures_detected;
        figure_text_spans_suppressed += fig_stats.spans_suppressed;
        all_figures.extend(figures.clone());

        let page_blocks = assemble_page_blocks(page, tables, figures, &mut current_peripheral);
        all_blocks.extend(page_blocks);
    }

    // 5. Stitch multi-page `(continued)` tables across consecutive pages
    let multi_page_tables_stitched = stitch_multipage_tables(&mut all_blocks);

    // 6. Compute bookmark-driven section shards and build section Markdown + TSV indexes
    let shards = compute_section_shards(&toc, metadata.total_pages);
    let indexes = build_section_files(&shards, &all_blocks, &metadata);

    // 7. Render cropped PNGs for detected figures
    let figures_rendered = render_detected_figures(&doc, config, &metadata.doc_stem, &all_figures)?;

    let elapsed_ms = start_ts.elapsed().as_millis();
    let sanity = compute_sanity_report(
        &indexes,
        SanityInputs {
            elapsed_ms,
            total_pages: metadata.total_pages,
            converted_pages: metadata.converted_pages,
            multi_page_tables_stitched,
            visual_bitboxes_deduplicated,
            figures_detected,
            figures_rendered,
            figure_text_spans_suppressed,
            header_footer_spans_stripped: clean_stats.header_footer_spans_stripped,
        },
    );

    // 8. Emit bundle files to disk
    write_agent_bundle(config, &metadata, &toc, &indexes, &sanity)
}
