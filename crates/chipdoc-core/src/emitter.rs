use crate::error::{ChipdocError, Result};
use crate::model::{
    ConversionBundle, ConvertConfig, DocMetadata, ExtractedTable, FigureRegion, PageBlock,
    RegisterBitfield, SanityReport, SectionFile, TocEntry,
};
use crate::registers::{extract_bitfields_from_table, infer_peripheral_name};
use crate::tables::render_table_markdown;
use std::fs;
use std::path::Path;

/// Intermediate indexed records collected across all shards for TSV emission.
#[derive(Debug, Default)]
pub struct BundleIndexes {
    pub sections: Vec<SectionFile>,
    pub tables: Vec<ExtractedTable>,
    pub registers: Vec<RegisterBitfield>,
    pub figures: Vec<FigureRegion>,
    pub unique_registers_count: usize,
}

/// Builds the Markdown content for each `SectionFile` shard and populates `BundleIndexes`.
pub fn build_section_files(
    shards: &[TocEntry],
    blocks: &[PageBlock],
    doc_meta: &DocMetadata,
) -> BundleIndexes {
    let mut indexes = BundleIndexes::default();
    let mut seen_registers = std::collections::HashSet::new();

    for (shard_idx, shard) in shards.iter().enumerate() {
        // Select blocks belonging to this shard's page range.
        // Note: if the next shard starts on the SAME page as this shard ends, avoid duplicating blocks.
        let shard_blocks: Vec<&PageBlock> = blocks
            .iter()
            .filter(|b| {
                let p = b.page();
                p >= shard.start_page && p <= shard.end_page
            })
            .collect();

        if shard_blocks.is_empty() {
            continue;
        }

        let file_rel_path = format!("sections/{}.md", shard.slug);
        let mut md_lines: Vec<String> = Vec::new();

        // YAML frontmatter for agent context
        md_lines.push("---".to_string());
        md_lines.push(format!(
            "document: \"{}\"",
            doc_meta
                .title
                .as_deref()
                .unwrap_or(&doc_meta.file_name)
                .replace('"', "\\\"")
        ));
        md_lines.push(format!("section_id: \"{}\"", shard.id));
        md_lines.push(format!(
            "section_title: \"{}\"",
            shard.title.replace('"', "\\\"")
        ));
        md_lines.push(format!(
            "pages: \"{}-{}\"",
            shard.start_page, shard.end_page
        ));
        md_lines.push("---".to_string());
        md_lines.push(String::new());
        md_lines.push(format!(
            "# {} (pp. {}-{})",
            shard.title, shard.start_page, shard.end_page
        ));
        md_lines.push(String::new());

        let mut active_peripheral = infer_peripheral_name(&shard.title);
        let mut active_register = String::new();
        let mut active_offset = String::new();
        let mut active_reset = String::new();

        let mut shard_tables = 0usize;
        let mut shard_regs = 0usize;
        let mut shard_figs = 0usize;

        for block in shard_blocks {
            match block {
                PageBlock::Heading {
                    level, text, page, ..
                } => {
                    let hashes = "#".repeat((*level + 1).clamp(2, 6));
                    let inferred = infer_peripheral_name(text);
                    if inferred != "SYS" && *level <= 2 {
                        active_peripheral = inferred;
                    }
                    md_lines.push(format!("{hashes} {text} (p. {page})"));
                    md_lines.push(String::new());
                }
                PageBlock::RegisterHeading {
                    section_num,
                    peripheral,
                    register,
                    offset,
                    reset,
                    page,
                    ..
                } => {
                    let periph = if peripheral != "SYS" && !peripheral.is_empty() {
                        peripheral.as_str()
                    } else {
                        active_peripheral.as_str()
                    };
                    active_register = register.clone();
                    active_offset = offset.clone();
                    active_reset = reset.clone();
                    seen_registers.insert(format!("{periph}.{register}"));
                    shard_regs += 1;

                    md_lines.push(format!(
                        "### {section_num} [{periph}.{register}] Offset: {offset} | Reset: {reset} (p. {page})"
                    ));
                    md_lines.push(String::new());
                }
                PageBlock::BitMapSummary { summary, .. } => {
                    md_lines.push(summary.clone());
                    md_lines.push(String::new());
                }
                PageBlock::Paragraph { text, .. } => {
                    md_lines.push(text.clone());
                    md_lines.push(String::new());
                }
                PageBlock::Table { table, .. } => {
                    let table_start_line = md_lines.len() + 1;
                    let rendered = render_table_markdown(table);
                    md_lines.extend(rendered);
                    md_lines.push(String::new());

                    let mut indexed_table = table.clone();
                    indexed_table.section_file = file_rel_path.clone();
                    indexed_table.line_number = table_start_line;

                    // Extract bitfields if this is a register field description table
                    let reg_name = if !active_register.is_empty() {
                        active_register.as_str()
                    } else {
                        indexed_table
                            .caption
                            .as_deref()
                            .and_then(|c| c.split_whitespace().nth(2))
                            .unwrap_or("REG")
                    };
                    let bitfields = extract_bitfields_from_table(
                        &indexed_table,
                        &active_peripheral,
                        reg_name,
                        if active_offset.is_empty() {
                            "N/A"
                        } else {
                            &active_offset
                        },
                        if active_reset.is_empty() {
                            "N/A"
                        } else {
                            &active_reset
                        },
                        &file_rel_path,
                        table_start_line,
                    );
                    indexes.registers.extend(bitfields);
                    indexes.tables.push(indexed_table);
                    shard_tables += 1;
                }
                PageBlock::Figure { figure, .. } => {
                    let fig_line = md_lines.len() + 1;
                    if let Some(rel_img) = &figure.image_rel_path {
                        md_lines.push(format!("![{}](../{})", figure.caption, rel_img));
                    }
                    md_lines.push(format!("*{} (p. {})*", figure.caption, figure.page_number));
                    md_lines.push(String::new());

                    let mut indexed_fig = figure.clone();
                    indexed_fig.section_file = file_rel_path.clone();
                    indexed_fig.line_number = fig_line;
                    indexes.figures.push(indexed_fig);
                    shard_figs += 1;
                }
            }
        }

        let line_count = md_lines.len();
        let markdown = md_lines.join("\n");

        indexes.sections.push(SectionFile {
            index: shard_idx + 1,
            section_id: shard.id.clone(),
            title: shard.title.clone(),
            level: shard.level,
            start_page: shard.start_page,
            end_page: shard.end_page,
            file_rel_path,
            markdown,
            line_count,
            table_count: shard_tables,
            register_count: shard_regs,
            figure_count: shard_figs,
        });
    }

    indexes.unique_registers_count = seen_registers.len();
    indexes
}

/// Writes the complete Agent Documentation Bundle (`INDEX.md`, `sections/*.md`, `index/*.tsv`,
/// `meta.json`, and optional `<doc_stem>.md`) to `<output_root>/<doc_stem>/`.
pub fn write_agent_bundle(
    config: &ConvertConfig,
    doc_meta: &DocMetadata,
    toc: &[TocEntry],
    indexes: &BundleIndexes,
    sanity: &SanityReport,
) -> Result<ConversionBundle> {
    let bundle_dir = config.output_root.join(&doc_meta.doc_stem);
    let sections_dir = bundle_dir.join("sections");
    let index_dir = bundle_dir.join("index");
    let figures_dir = bundle_dir.join("assets").join("figures");

    create_dir(&bundle_dir)?;
    create_dir(&sections_dir)?;
    create_dir(&index_dir)?;
    if config.render_figures {
        create_dir(&figures_dir)?;
    }

    // 1. Write `sections/*.md`
    for sec in &indexes.sections {
        let out_path = bundle_dir.join(&sec.file_rel_path);
        write_file(&out_path, &sec.markdown)?;
    }

    // 2. Write `index/sections.tsv`
    let mut sec_tsv =
        String::from("section_id\ttitle\tfile_path\tlines\tpages\ttables\tregisters\tfigures\n");
    for sec in &indexes.sections {
        sec_tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}-{}\t{}\t{}\t{}\n",
            sec.section_id,
            sanitize_tsv(&sec.title),
            sec.file_rel_path,
            sec.line_count,
            sec.start_page,
            sec.end_page,
            sec.table_count,
            sec.register_count,
            sec.figure_count,
        ));
    }
    write_file(&index_dir.join("sections.tsv"), &sec_tsv)?;

    // 3. Write `index/tables.tsv`
    let mut tab_tsv = String::from("table_id\tcaption\tcolumns\trows\tlocation\tpages\n");
    for tb in &indexes.tables {
        let cap = tb.caption.as_deref().unwrap_or(&tb.id);
        let cols = tb.headers.join(" | ");
        let pages = if tb.page_start == tb.page_end {
            format!("{}", tb.page_start)
        } else {
            format!("{}-{}", tb.page_start, tb.page_end)
        };
        tab_tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}:{}\t{}\n",
            tb.id,
            sanitize_tsv(cap),
            sanitize_tsv(&cols),
            tb.rows.len(),
            tb.section_file,
            tb.line_number,
            pages
        ));
    }
    write_file(&index_dir.join("tables.tsv"), &tab_tsv)?;

    // 4. Write `index/registers.tsv`
    let mut reg_tsv = String::from(
        "peripheral\tregister\toffset\treset\tbits\tfield\taccess\tfield_reset\tdescription\tlocation\tpage\n",
    );
    for r in &indexes.registers {
        reg_tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}:{}\t{}\n",
            sanitize_tsv(&r.peripheral),
            sanitize_tsv(&r.register),
            sanitize_tsv(&r.offset),
            sanitize_tsv(&r.reset_value),
            sanitize_tsv(&r.bits),
            sanitize_tsv(&r.field_name),
            sanitize_tsv(&r.access),
            sanitize_tsv(&r.field_reset),
            sanitize_tsv(&r.description),
            r.section_file,
            r.line_number,
            r.page_number
        ));
    }
    write_file(&index_dir.join("registers.tsv"), &reg_tsv)?;

    // 5. Write `index/figures.tsv`
    let mut fig_tsv = String::from("figure_id\tcaption\timage_path\tlocation\tpage\n");
    for f in &indexes.figures {
        fig_tsv.push_str(&format!(
            "{}\t{}\t{}\t{}:{}\t{}\n",
            f.id,
            sanitize_tsv(&f.caption),
            f.image_rel_path.as_deref().unwrap_or(""),
            f.section_file,
            f.line_number,
            f.page_number
        ));
    }
    write_file(&index_dir.join("figures.tsv"), &fig_tsv)?;

    // 6. Write `INDEX.md`
    let index_md = build_master_index_markdown(doc_meta, toc, indexes, sanity);
    let index_md_path = bundle_dir.join("INDEX.md");
    write_file(&index_md_path, &index_md)?;

    // 7. Write `meta.json`
    let meta_json = serde_json::to_string_pretty(&serde_json::json!({
        "metadata": doc_meta,
        "sanity": sanity,
    }))?;
    write_file(&bundle_dir.join("meta.json"), &meta_json)?;

    // 8. Optionally write single consolidated `.md` file
    let single_md_path = if config.single_file {
        let mut combined = String::new();
        combined.push_str(&index_md);
        combined.push_str("\n---\n\n");
        for sec in &indexes.sections {
            combined.push_str(&sec.markdown);
            combined.push_str("\n\n---\n\n");
        }
        let path = bundle_dir.join(format!("{}.md", doc_meta.doc_stem));
        write_file(&path, &combined)?;
        Some(path)
    } else {
        None
    };

    Ok(ConversionBundle {
        bundle_dir,
        index_md_path,
        single_md_path,
        metadata: doc_meta.clone(),
        sanity: sanity.clone(),
    })
}

fn build_master_index_markdown(
    doc_meta: &DocMetadata,
    toc: &[TocEntry],
    indexes: &BundleIndexes,
    sanity: &SanityReport,
) -> String {
    let mut lines = Vec::new();
    let title = doc_meta.title.as_deref().unwrap_or(&doc_meta.file_name);
    lines.push(format!("# {title}"));
    lines.push(String::new());
    lines.push("> **Agent Navigation Guide**:".to_string());
    lines.push(
        "> - For fast 1-line-per-bitfield register lookups, grep `index/registers.tsv`."
            .to_string(),
    );
    lines.push(
        "> - For electrical specification or summary tables, grep `index/tables.tsv`.".to_string(),
    );
    lines.push(
        "> - For chapter/peripheral details, grep `sections/` or open the corresponding file below."
            .to_string(),
    );
    lines.push(String::new());

    lines.push("## Document Summary".to_string());
    lines.push(String::new());
    lines.push(format!("- **Source File**: `{}`", doc_meta.file_name));
    if let Some(author) = &doc_meta.author {
        lines.push(format!("- **Author**: {}", author.trim()));
    }
    if let Some(kw) = &doc_meta.keywords {
        lines.push(format!("- **Document ID / Keywords**: `{}`", kw.trim()));
    }
    lines.push(format!(
        "- **Pages Converted**: {} / {}",
        doc_meta.converted_pages, doc_meta.total_pages
    ));
    lines.push(format!(
        "- **Section Files**: {} (`sections/*.md`)",
        indexes.sections.len()
    ));
    lines.push(format!(
        "- **Tables Extracted**: {} ({} stitched across pages) -> `index/tables.tsv`",
        sanity.tables_extracted, sanity.multi_page_tables_stitched
    ));
    lines.push(format!(
        "- **Registers / Bitfields Indexed**: {} registers / {} bitfields -> `index/registers.tsv`",
        sanity.registers_indexed, sanity.bitfields_indexed
    ));
    lines.push(format!(
        "- **Figures Detected**: {} ({} rendered to `assets/figures/`) -> `index/figures.tsv`",
        sanity.figures_detected, sanity.figures_rendered
    ));
    lines.push(format!(
        "- **Sanity Score**: {}/100 (~{} tokens)",
        sanity.sanity_score, sanity.estimated_tokens
    ));
    lines.push(String::new());

    lines.push("## Section Shards (`sections/*.md`)".to_string());
    lines.push(String::new());
    lines.push(
        "| ID | Section Title | File | Pages | Lines | Tables | Registers | Figures |".to_string(),
    );
    lines.push("| --- | --- | --- | --- | ---: | ---: | ---: | ---: |".to_string());
    for sec in &indexes.sections {
        lines.push(format!(
            "| `{}` | {} | [{}]({}) | {}-{} | {} | {} | {} | {} |",
            sec.section_id,
            sec.title.replace('|', "\\|"),
            sec.file_rel_path,
            sec.file_rel_path,
            sec.start_page,
            sec.end_page,
            sec.line_count,
            sec.table_count,
            sec.register_count,
            sec.figure_count
        ));
    }
    lines.push(String::new());

    if !toc.is_empty() {
        lines.push("## Detailed Outline (Table of Contents)".to_string());
        lines.push(String::new());
        for entry in toc.iter().filter(|e| e.level <= 2) {
            let indent = "  ".repeat(entry.level);
            // Find which section file covers `entry.start_page`
            let shard_link = indexes
                .sections
                .iter()
                .find(|s| entry.start_page >= s.start_page && entry.start_page <= s.end_page)
                .map(|s| format!(" (`{}`)", s.file_rel_path))
                .unwrap_or_default();
            lines.push(format!(
                "{indent}- **{}** — p. {}{shard_link}",
                entry.title, entry.start_page
            ));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

fn sanitize_tsv(s: &str) -> String {
    s.replace(['\t', '\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).map_err(|source| ChipdocError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content.as_bytes()).map_err(|source| ChipdocError::Io {
        path: path.to_path_buf(),
        source,
    })
}
