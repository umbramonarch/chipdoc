use anyhow::{Context, Result};
use chipdoc_core::model::ConvertConfig;
use chipdoc_core::pdf_loader::parse_page_range_expr;
use chipdoc_core::query::{
    parallel_bundle_search, query_figures_formatted, query_pages_formatted,
    query_registers_formatted, query_sections_formatted, query_tables_formatted,
    query_xref_formatted, resolve_bundles, scan_workspace_status,
};
use chipdoc_core::{convert_pdf, inspect_pdf};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(
    name = "chipdoc",
    version,
    about = "Agent Document Intelligence CLI: convert, index, and query technical PDFs (datasheets, TRMs, errata)"
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

        /// Index figure bounding boxes in `figures.tsv` immediately and defer PNG rasterization until requested
        #[arg(long)]
        lazy_figures: bool,

        /// Force reconversion even when an up-to-date bundle already exists in `<out>`
        #[arg(long)]
        force: bool,

        /// Do not automatically add `<out>/` to `.claudeignore`
        #[arg(long)]
        no_claudeignore: bool,

        /// DPI resolution for rendering cropped figure PNGs
        #[arg(long, default_value_t = 200.0)]
        dpi: f32,
    },

    /// Ensure one or more PDFs are converted and cached (fast no-op if up to date)
    Ensure {
        /// Input PDF file path(s)
        #[arg(required = true)]
        inputs: Vec<PathBuf>,

        /// Output root directory
        #[arg(short, long, default_value = "output")]
        out: PathBuf,

        /// Defer figure PNG rendering until requested via `chipdoc figure --render`
        #[arg(long)]
        lazy_figures: bool,
    },

    /// Show converted bundles and unconverted PDFs in the workspace
    Status {
        /// Workspace or output directory to scan
        #[arg(default_value = ".")]
        workspace: PathBuf,

        /// Emit a single-block compact summary (ideal for SessionStart hooks)
        #[arg(long)]
        compact: bool,
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

    /// Look up registers and bitfields by name (`FSUB_0`, `GPIO.FSUB_0`), bitfield, or hex offset (`0x400`, `400h`)
    Reg {
        /// Either `<query>` (searches all bundles in `./output`) or `<bundle_dir>` followed by `<query>`
        arg1: String,

        /// Optional `<query>` when `arg1` is a bundle path
        arg2: Option<String>,

        /// Explicit bundle or root directory to search
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Maximum register cards to return
        #[arg(short, long, default_value_t = 10)]
        limit: usize,

        /// Truncate long bitfield descriptions for minimal token usage
        #[arg(long)]
        compact: bool,
    },

    /// Search tables by caption, column, or ID and optionally display the Markdown table inline
    Table {
        /// Either `<query>` or `<bundle_dir>` followed by `<query>`
        arg1: String,

        /// Optional `<query>` when `arg1` is a bundle path
        arg2: Option<String>,

        /// Explicit bundle or root directory to search
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Print the reconstructed Markdown table rows inline
        #[arg(long)]
        show: bool,

        /// Maximum rows to display per inline Markdown table
        #[arg(long, default_value_t = 35)]
        max_rows: usize,

        /// Maximum matching tables to list
        #[arg(short, long, default_value_t = 15)]
        limit: usize,
    },

    /// List or read chapter/section shards by section number (`10.3`), slug (`gpio`), or keyword
    Section {
        /// Optional `<query>` or `<bundle_dir>` followed by `<query>`
        #[arg(default_value = "")]
        arg1: String,

        /// Optional `<query>` when `arg1` is a bundle path
        arg2: Option<String>,

        /// Explicit bundle or root directory to search
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Maximum Markdown lines to preview when a section matches (0 = list metadata only)
        #[arg(long, default_value_t = 100)]
        max_lines: usize,
    },

    /// Extract Markdown blocks corresponding to 1-based original PDF page numbers (e.g. `564` or `564-565`)
    Page {
        /// Either `<page_expr>` (if only 1 bundle exists) or `<bundle_dir>` followed by `<page_expr>`
        arg1: String,

        /// Optional `<page_expr>` when `arg1` is a bundle path
        arg2: Option<String>,

        /// Explicit bundle directory
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Maximum lines to print
        #[arg(long, default_value_t = 120)]
        max_lines: usize,
    },

    /// Traverse the Hardware Cross-Reference Graph (`index/xref.tsv`) for a peripheral, register, pin, or symbol
    #[command(visible_alias = "explain")]
    Xref {
        /// Either `<symbol>` or `<bundle_dir>` followed by `<symbol>`
        arg1: String,

        /// Optional `<symbol>` when `arg1` is a bundle path
        arg2: Option<String>,

        /// Explicit bundle or root directory to search
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Maximum cross-reference edges to print per bundle
        #[arg(short, long, default_value_t = 30)]
        limit: usize,
    },

    /// Search detected figures and optionally render cropped PNGs on demand (`--render <pdf>`)
    Figure {
        /// Either `<query>` or `<bundle_dir>` followed by `<query>`
        #[arg(default_value = "")]
        arg1: String,

        /// Optional `<query>` when `arg1` is a bundle path
        arg2: Option<String>,

        /// Explicit bundle or root directory to search
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Source PDF path to render cropped PNGs on demand (when converted with `--lazy-figures`)
        #[arg(long)]
        render: Option<PathBuf>,

        /// DPI resolution for on-demand figure rendering
        #[arg(long, default_value_t = 200.0)]
        dpi: f32,

        /// Maximum figures to list
        #[arg(short, long, default_value_t = 25)]
        limit: usize,
    },

    /// Parallel SIMD + Regex search across one or all generated Agent Documentation Bundles
    Search {
        /// Either `<pattern>` (searches `./output`) or `<bundle_or_root_dir>` followed by `<pattern>`
        arg1: String,

        /// Optional `<pattern>` when `arg1` is a bundle or root directory
        arg2: Option<String>,

        /// Explicit bundle or root directory to search
        #[arg(short, long)]
        bundle: Option<PathBuf>,

        /// Filter search target (`all`, `registers`, `tables`, `figures`, `xref`, `sections`)
        #[arg(short, long, default_value = "all")]
        kind: String,

        /// Case-insensitive matching
        #[arg(short, long)]
        ignore_case: bool,

        /// Maximum matches to print per category
        #[arg(short, long, default_value_t = 25)]
        limit: usize,

        /// Truncate long lines for minimal token usage
        #[arg(long)]
        compact: bool,
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
            lazy_figures,
            force,
            no_claudeignore,
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
                    lazy_figures,
                    force,
                    update_claudeignore: !no_claudeignore,
                    figure_dpi: dpi,
                    pdfium_lib_dir: cli.pdfium_dir.clone(),
                };

                let bundle = convert_pdf(&cfg)
                    .with_context(|| format!("Failed to convert '{}'", input_pdf.display()))?;

                print_conversion_summary(&bundle);
            }
        }

        Commands::Ensure {
            inputs,
            out,
            lazy_figures,
        } => {
            for input_pdf in inputs {
                let mut cfg = ConvertConfig::new(input_pdf.clone(), out.clone());
                cfg.lazy_figures = lazy_figures;
                cfg.pdfium_lib_dir = cli.pdfium_dir.clone();

                let bundle = convert_pdf(&cfg)
                    .with_context(|| format!("Failed to ensure '{}'", input_pdf.display()))?;
                let tag = if bundle.sanity.cache_hit {
                    "cached"
                } else {
                    "converted"
                };
                println!(
                    "[{tag}] {} -> {} ({} pp, {} regs, {} tables, {} xrefs in {}ms)",
                    input_pdf.display(),
                    bundle.bundle_dir.display(),
                    bundle.sanity.converted_pages,
                    bundle.sanity.registers_indexed,
                    bundle.sanity.tables_extracted,
                    bundle.sanity.xrefs_indexed,
                    bundle.sanity.elapsed_ms,
                );
            }
        }

        Commands::Status { workspace, compact } => {
            let st = scan_workspace_status(&workspace);
            if compact {
                if st.bundles.is_empty() && st.unconverted_pdfs.is_empty() {
                    return Ok(());
                }
                if !st.bundles.is_empty() {
                    let items: Vec<String> = st
                        .bundles
                        .iter()
                        .map(|b| {
                            format!(
                                "`{}` ({}pp, {} regs, {} tables)",
                                b.bundle_dir.display(),
                                b.metadata.converted_pages,
                                b.sanity.registers_indexed,
                                b.sanity.tables_extracted
                            )
                        })
                        .collect();
                    println!("chipdoc bundles ready: {}", items.join(", "));
                }
                if !st.unconverted_pdfs.is_empty() {
                    let pdfs: Vec<String> = st
                        .unconverted_pdfs
                        .iter()
                        .map(|p| format!("`{}`", p.display()))
                        .collect();
                    println!(
                        "unconverted PDFs (run `chipdoc ensure <pdf>` before reading): {}",
                        pdfs.join(", ")
                    );
                }
                println!("Prefer 1-turn CLI queries: `chipdoc reg <name|hex>`, `chipdoc table <q> --show`, `chipdoc xref <sym>`, `chipdoc page <N>`.");
                return Ok(());
            }

            println!("=== chipdoc Workspace Status ({}) ===", workspace.display());
            if st.bundles.is_empty() {
                println!("Converted Bundles : (none)");
            } else {
                println!("Converted Bundles ({}):", st.bundles.len());
                for b in &st.bundles {
                    println!(
                        "  [ready] {:<28} | {:>4} pp | {:>4} regs ({:>5} bitfields) | {:>4} tables | {:>5} xrefs | {}",
                        b.metadata.doc_stem,
                        b.metadata.converted_pages,
                        b.sanity.registers_indexed,
                        b.sanity.bitfields_indexed,
                        b.sanity.tables_extracted,
                        b.sanity.xrefs_indexed,
                        b.bundle_dir.display()
                    );
                }
            }
            if !st.unconverted_pdfs.is_empty() {
                println!("\nUnconverted PDFs ({}):", st.unconverted_pdfs.len());
                for p in &st.unconverted_pdfs {
                    println!(
                        "  [pending] {} (run `chipdoc ensure \"{}\"`)",
                        p.display(),
                        p.display()
                    );
                }
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
                "Fingerprint    : {} ({} bytes)",
                info.metadata.source_hash, info.metadata.source_size_bytes
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

        Commands::Reg {
            arg1,
            arg2,
            bundle,
            limit,
            compact,
        } => {
            let (bundles, query) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let out = query_registers_formatted(&bundles, &query, limit, compact)?;
            print!("{out}");
        }

        Commands::Table {
            arg1,
            arg2,
            bundle,
            show,
            max_rows,
            limit,
        } => {
            let (bundles, query) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let out = query_tables_formatted(&bundles, &query, show, max_rows, limit)?;
            print!("{out}");
        }

        Commands::Section {
            arg1,
            arg2,
            bundle,
            max_lines,
        } => {
            let (bundles, query) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let out = query_sections_formatted(&bundles, &query, max_lines)?;
            print!("{out}");
        }

        Commands::Page {
            arg1,
            arg2,
            bundle,
            max_lines,
        } => {
            let (bundles, page_expr) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let target_bundle = bundles
                .first()
                .context("No converted bundle found for page lookup")?;
            let out = query_pages_formatted(target_bundle, &page_expr, max_lines)?;
            print!("{out}");
        }

        Commands::Xref {
            arg1,
            arg2,
            bundle,
            limit,
        } => {
            let (bundles, symbol) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let out = query_xref_formatted(&bundles, &symbol, limit)?;
            print!("{out}");
        }

        Commands::Figure {
            arg1,
            arg2,
            bundle,
            render,
            dpi,
            limit,
        } => {
            let (bundles, query) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let out = query_figures_formatted(
                &bundles,
                &query,
                render.as_deref(),
                dpi,
                cli.pdfium_dir.as_deref(),
                limit,
            )?;
            print!("{out}");
        }

        Commands::Search {
            arg1,
            arg2,
            bundle,
            kind,
            ignore_case,
            limit,
            compact,
        } => {
            let (bundles, pattern) =
                resolve_target_and_query(&arg1, arg2.as_deref(), bundle.as_deref())?;
            let out =
                parallel_bundle_search(&bundles, &pattern, &kind, ignore_case, limit, compact)?;
            print!("{out}");
        }
    }

    Ok(())
}

/// Supports both `chipdoc <cmd> <bundle_dir> <query>` and `chipdoc <cmd> <query> [-b <bundle_dir>]`.
fn resolve_target_and_query(
    arg1: &str,
    arg2: Option<&str>,
    explicit_bundle: Option<&Path>,
) -> Result<(Vec<PathBuf>, String)> {
    if let Some(b) = explicit_bundle {
        let bundles = resolve_bundles(b)?;
        return Ok((bundles, arg1.to_string()));
    }
    if let Some(q) = arg2 {
        let bundles = resolve_bundles(Path::new(arg1))?;
        return Ok((bundles, q.to_string()));
    }
    // Single positional argument: check if arg1 itself is a bundle directory (for `section` / `figure` listing)
    let p1 = Path::new(arg1);
    if !arg1.is_empty()
        && p1.is_dir()
        && (p1.join("meta.json").exists() || resolve_bundles(p1).is_ok())
    {
        let bundles = resolve_bundles(p1)?;
        return Ok((bundles, String::new()));
    }
    let bundles = resolve_bundles(Path::new("."))?;
    Ok((bundles, arg1.to_string()))
}

fn print_conversion_summary(bundle: &chipdoc_core::model::ConversionBundle) {
    let s = &bundle.sanity;
    let mode_tag = if s.cache_hit {
        "cached hit"
    } else {
        "converted"
    };
    println!(
        "    [ok] Bundle      : {} ({mode_tag})",
        bundle.bundle_dir.display()
    );
    println!("    [ok] Master Index: {}", bundle.index_md_path.display());
    println!(
        "    [ok] Pages       : {}/{} in {:.2}s ({} sections)",
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
        "    [ok] Cross-Refs  : {} hardware symbols linked in index/xref.tsv",
        s.xrefs_indexed
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
