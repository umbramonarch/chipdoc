use anyhow::{Context, Result};
use chipdoc_core::model::ConvertConfig;
use chipdoc_core::pdf_loader::parse_page_range_expr;
use chipdoc_core::{convert_pdf, inspect_pdf};
use clap::{Parser, Subcommand};
use regex::RegexBuilder;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(
    name = "chipdoc",
    version,
    about = "Convert technical and embedded PDFs (datasheets, TRMs) into grep-ready Agent Documentation Bundles"
)]
struct Cli {
    /// Optional directory containing pdfium.dll / libpdfium.so / libpdfium.dylib
    #[arg(long, global = true)]
    pdfium_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Convert one or more PDF files into Hierarchical Agent Documentation Bundles
    Convert {
        /// Input PDF file path(s)
        #[arg(required = true)]
        inputs: Vec<PathBuf>,

        /// Output root directory (each PDF creates `<out>/<pdf_stem>/`)
        #[arg(short, long, default_value = "output")]
        out: PathBuf,

        /// Optional 1-based page range filter (e.g. `1-20,41,562-565`)
        #[arg(long)]
        pages: Option<String>,

        /// Also emit a single merged `<pdf_stem>.md` file alongside `sections/*.md`
        #[arg(long)]
        single_file: bool,

        /// Skip rendering cropped figure PNGs to `assets/figures/`
        #[arg(long)]
        no_figures: bool,

        /// DPI resolution for rendering cropped figure PNGs
        #[arg(long, default_value_t = 200.0)]
        dpi: f32,
    },

    /// Inspect a PDF's metadata, Table of Contents, and geometric object counts
    Inspect {
        /// Input PDF file path
        input: PathBuf,

        /// Optional 1-based page range filter (e.g. `1-10`)
        #[arg(long)]
        pages: Option<String>,

        /// Maximum TOC entries to display
        #[arg(long, default_value_t = 40)]
        max_toc: usize,
    },

    /// Search across a generated Agent Documentation Bundle (`index/*.tsv` and `sections/*.md`)
    Search {
        /// Path to the generated bundle directory (e.g. `output/mspm0c1105_trm`)
        bundle_dir: PathBuf,

        /// Regex or literal query pattern
        pattern: String,

        /// Filter search target (`all`, `registers`, `tables`, `figures`, `sections`)
        #[arg(short, long, default_value = "all")]
        kind: String,

        /// Case-insensitive matching
        #[arg(short, long)]
        ignore_case: bool,

        /// Maximum matches to print per category
        #[arg(short, long, default_value_t = 25)]
        limit: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert {
            inputs,
            out,
            pages,
            single_file,
            no_figures,
            dpi,
        } => {
            let page_filter = match pages {
                Some(ref expr) => Some(parse_page_range_expr(expr)?),
                None => None,
            };

            for input_pdf in inputs {
                println!("==> Converting '{}' ...", input_pdf.display());
                let cfg = ConvertConfig {
                    input_pdf: input_pdf.clone(),
                    output_root: out.clone(),
                    page_filter: page_filter.clone(),
                    single_file,
                    render_figures: !no_figures,
                    figure_dpi: dpi,
                    pdfium_lib_dir: cli.pdfium_dir.clone(),
                };

                let bundle = convert_pdf(&cfg)
                    .with_context(|| format!("Failed to convert '{}'", input_pdf.display()))?;

                let s = &bundle.sanity;
                println!("    [ok] Bundle      : {}", bundle.bundle_dir.display());
                println!("    [ok] Master Index: {}", bundle.index_md_path.display());
                println!(
                    "    [ok] Pages       : {}/{} converted in {:.2}s ({} sections)",
                    s.converted_pages,
                    s.total_pages,
                    (s.elapsed_ms as f64) / 1000.0,
                    s.section_files_count
                );
                println!(
                    "    [ok] Tables      : {} extracted ({} multi-page continuations stitched)",
                    s.tables_extracted, s.multi_page_tables_stitched
                );
                println!(
                    "    [ok] Registers   : {} registers / {} bitfields indexed ({} visual bit-boxes deduplicated)",
                    s.registers_indexed, s.bitfields_indexed, s.visual_bitboxes_deduplicated
                );
                println!(
                    "    [ok] Figures     : {} detected / {} PNGs rendered ({} floating diagram spans suppressed)",
                    s.figures_detected, s.figures_rendered, s.figure_text_spans_suppressed
                );
                println!(
                    "    [ok] Sanity      : {}/100 (~{} tokens, {} header/footer spans stripped)",
                    s.sanity_score, s.estimated_tokens, s.header_footer_spans_stripped
                );
            }
        }

        Commands::Inspect {
            input,
            pages,
            max_toc,
        } => {
            let page_filter = match pages {
                Some(ref expr) => Some(parse_page_range_expr(expr)?),
                None => None,
            };
            let info = inspect_pdf(&input, page_filter.as_deref(), cli.pdfium_dir.as_deref())?;

            println!("=== PDF Inspection: {} ===", info.metadata.file_name);
            println!(
                "Title          : {}",
                info.metadata.title.as_deref().unwrap_or("-")
            );
            println!(
                "Author         : {}",
                info.metadata.author.as_deref().unwrap_or("-")
            );
            println!(
                "Subject        : {}",
                info.metadata.subject.as_deref().unwrap_or("-")
            );
            println!(
                "Keywords       : {}",
                info.metadata.keywords.as_deref().unwrap_or("-")
            );
            println!(
                "Pages          : {} (inspected {})",
                info.metadata.total_pages, info.metadata.converted_pages
            );
            println!("Text Spans     : {}", info.total_spans);
            println!("Grid Segments  : {}", info.total_grid_segments);
            println!("Vector Boxes   : {}", info.total_graphics);
            println!("Raster Images  : {}", info.total_images);
            println!("TOC Bookmarks  : {}", info.toc.len());

            for entry in info.toc.iter().take(max_toc) {
                let indent = "  ".repeat(entry.level);
                println!(
                    "  {indent}- {} (pp. {}-{})",
                    entry.title, entry.start_page, entry.end_page
                );
            }
            if info.toc.len() > max_toc {
                println!("  ... ({} more bookmarks)", info.toc.len() - max_toc);
            }
        }

        Commands::Search {
            bundle_dir,
            pattern,
            kind,
            ignore_case,
            limit,
        } => {
            run_bundle_search(&bundle_dir, &pattern, &kind, ignore_case, limit)?;
        }
    }

    Ok(())
}

fn run_bundle_search(
    bundle_dir: &Path,
    pattern: &str,
    kind: &str,
    ignore_case: bool,
    limit: usize,
) -> Result<()> {
    let re = RegexBuilder::new(pattern)
        .case_insensitive(ignore_case)
        .build()
        .with_context(|| format!("Invalid regex pattern '{pattern}'"))?;

    let k = kind.trim().to_ascii_lowercase();
    let search_regs = k == "all" || k.starts_with("reg");
    let search_tabs = k == "all" || k.starts_with("tab");
    let search_figs = k == "all" || k.starts_with("fig");
    let search_secs = k == "all" || k.starts_with("sec") || k == "md";

    // 1. Search `index/registers.tsv`
    let reg_tsv = bundle_dir.join("index").join("registers.tsv");
    if search_regs && reg_tsv.exists() {
        let content = fs::read_to_string(&reg_tsv)?;
        let matches: Vec<&str> = content
            .lines()
            .skip(1)
            .filter(|line| re.is_match(line))
            .take(limit)
            .collect();
        if !matches.is_empty() {
            println!("=== Register / Bitfield Matches (`index/registers.tsv`) ===");
            for m in matches {
                println!("  {m}");
            }
            println!();
        }
    }

    // 2. Search `index/tables.tsv`
    let tab_tsv = bundle_dir.join("index").join("tables.tsv");
    if search_tabs && tab_tsv.exists() {
        let content = fs::read_to_string(&tab_tsv)?;
        let matches: Vec<&str> = content
            .lines()
            .skip(1)
            .filter(|line| re.is_match(line))
            .take(limit)
            .collect();
        if !matches.is_empty() {
            println!("=== Table Matches (`index/tables.tsv`) ===");
            for m in matches {
                println!("  {m}");
            }
            println!();
        }
    }

    // 3. Search `index/figures.tsv`
    let fig_tsv = bundle_dir.join("index").join("figures.tsv");
    if search_figs && fig_tsv.exists() {
        let content = fs::read_to_string(&fig_tsv)?;
        let matches: Vec<&str> = content
            .lines()
            .skip(1)
            .filter(|line| re.is_match(line))
            .take(limit)
            .collect();
        if !matches.is_empty() {
            println!("=== Figure Matches (`index/figures.tsv`) ===");
            for m in matches {
                println!("  {m}");
            }
            println!();
        }
    }

    // 4. Search `sections/*.md`
    let sections_dir = bundle_dir.join("sections");
    if search_secs && sections_dir.exists() {
        println!("=== Markdown Section Matches (`sections/*.md`) ===");
        let mut count = 0usize;
        let mut entries: Vec<_> = fs::read_dir(&sections_dir)?
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| e.file_name());

        'outer: for entry in entries {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let rel_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("section.md");
            let content = fs::read_to_string(&path)?;
            for (line_idx, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    println!("  sections/{}:{}: {}", rel_name, line_idx + 1, line);
                    count += 1;
                    if count >= limit {
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(())
}
