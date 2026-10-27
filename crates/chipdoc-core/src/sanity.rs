use crate::emitter::BundleIndexes;
use crate::model::SanityReport;

/// Inputs used to compute the conversion sanity report.
#[derive(Debug, Clone, Copy)]
pub struct SanityInputs {
    pub elapsed_ms: u128,
    pub total_pages: usize,
    pub converted_pages: usize,
    pub multi_page_tables_stitched: usize,
    pub visual_bitboxes_deduplicated: usize,
    pub figures_detected: usize,
    pub figures_rendered: usize,
    pub figure_text_spans_suppressed: usize,
    pub header_footer_spans_stripped: usize,
}

/// Evaluates the generated bundle for structural integrity, broken glyphs, and token estimate.
pub fn compute_sanity_report(indexes: &BundleIndexes, inputs: SanityInputs) -> SanityReport {
    let mut total_chars = 0usize;
    let mut replacement_char_count = 0usize;
    let mut heading_count = 0usize;
    let mut has_raw_sub_sup_html = false;
    let mut has_table_or_figure_section_title = false;

    for sec in &indexes.sections {
        total_chars += sec.markdown.len();
        replacement_char_count += sec.markdown.chars().filter(|&c| c == '\u{FFFD}').count();
        heading_count += sec.markdown.lines().filter(|l| l.starts_with('#')).count();

        let md_lower = sec.markdown.to_ascii_lowercase();
        if md_lower.contains("<sup")
            || md_lower.contains("</sup")
            || md_lower.contains("<sub")
            || md_lower.contains("</sub")
        {
            has_raw_sub_sup_html = true;
        }

        let trimmed_title = sec.title.trim();
        if trimmed_title
            .strip_prefix("Table ")
            .or_else(|| trimmed_title.strip_prefix("Figure "))
            .is_some_and(|rest| rest.chars().next().is_some_and(|c| c.is_ascii_digit()))
        {
            has_table_or_figure_section_title = true;
        }
    }

    let estimated_tokens = (total_chars as f64 / 3.8).ceil() as usize;
    let mut score: i32 = 100;
    let mut warnings = Vec::new();

    if indexes.sections.is_empty() {
        score -= 60;
        warnings.push(
            "No section files were generated (document may be scanned without OCR).".to_string(),
        );
    }
    if heading_count == 0 {
        score -= 20;
        warnings.push("No Markdown headings were detected.".to_string());
    }
    if replacement_char_count > 25 {
        score -= 10;
        warnings.push(format!(
            "Detected {replacement_char_count} Unicode replacement characters (U+FFFD)."
        ));
    }
    if has_raw_sub_sup_html {
        score -= 5;
        warnings.push("Detected raw inline HTML <sub>/<sup> tags in section Markdown.".to_string());
    }
    if has_table_or_figure_section_title {
        score -= 10;
        warnings.push(
            "Detected top-level section title starting with 'Table ' or 'Figure '.".to_string(),
        );
    }

    SanityReport {
        chipdoc_version: env!("CARGO_PKG_VERSION").to_string(),
        elapsed_ms: inputs.elapsed_ms,
        total_pages: inputs.total_pages,
        converted_pages: inputs.converted_pages,
        section_files_count: indexes.sections.len(),
        tables_extracted: indexes.tables.len(),
        multi_page_tables_stitched: inputs.multi_page_tables_stitched,
        registers_indexed: indexes.unique_registers_count,
        bitfields_indexed: indexes.registers.len(),
        visual_bitboxes_deduplicated: inputs.visual_bitboxes_deduplicated,
        figures_detected: inputs.figures_detected,
        figures_rendered: inputs.figures_rendered,
        figure_text_spans_suppressed: inputs.figure_text_spans_suppressed,
        header_footer_spans_stripped: inputs.header_footer_spans_stripped,
        xrefs_indexed: indexes.xrefs.len(),
        cache_hit: false,
        replacement_char_count,
        estimated_tokens,
        sanity_score: score.clamp(0, 100) as u8,
        warnings,
    }
}
