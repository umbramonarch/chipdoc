# `chipdoc` — PDF-to-Agent Technical Document Converter

`chipdoc` is a fast, vendor-neutral **Rust CLI and library** that converts microcontroller datasheets, Technical Reference Manuals (TRMs), errata, and engineering PDFs into a **Hierarchical Agent Documentation Bundle** designed specifically for AI coding agents (`Grep` / `ripgrep` + `Read`).

---

## Why `chipdoc`?

General-purpose PDF-to-Markdown tools (`markitdown`, `docling`, `pymupdf4llm`) struggle with embedded-systems documentation for six concrete reasons:

1. **Vector Diagrams Become Scrambled Text**: Pinout diagrams, block diagrams, and timing charts in datasheets are drawn using **PDF vector paths + floating text labels**, not bitmap images. Naive extractors dump hundreds of floating pin labels (`PA0`, `VDD`, `36`, `PB17`) into the middle of Markdown prose in arbitrary Y-coordinate order. `chipdoc` detects figure bounding boxes, **suppresses floating diagram text from polluting Markdown**, and **renders cropped figure PNGs** to `assets/figures/`.
2. **Redundant Visual Register Bit-Boxes**: Every register in a TRM includes a 16- or 32-bit visual bit-grid figure immediately above a `Field Descriptions` table with the exact same information. `chipdoc` condenses the noisy visual bit-grid into a 1-line bit-map summary and preserves the clean Markdown table, cutting register-page token usage by **30–40%**.
3. **Complex Multi-Header & Multi-Page Tables**: Electrical characteristic tables use 2-level merged headers (`-40°C / 25°C / 85°C` $\times$ `TYP / MAX`) and mode divider rows (`RUN Mode`, `SLEEP Mode`), while pinout and register tables span many pages with `(continued)` headers and mid-identifier line wraps (`GEN_EVENT\n0`). `chipdoc` reconstructs cell grids from PDF vector geometry, flattens multi-row headers, repairs wrapped identifiers, drops hyperlink filler columns (`Section: Go`), and stitches `(continued)` tables across page breaks.
4. **Unicode & Subscript Kerning Traps**: Datasheets frequently use Unicode en-dashes for negative numbers (`–0.3 V`), single-glyph Celsius symbols (`℃`), and tight subscript kerning that fuses symbols into adjacent words (`VIInput voltage`), causing exact `grep` queries to miss. `chipdoc` normalizes these deterministically (`-0.3 V`, `°C`, `VI Input voltage`).
5. **Running Header/Footer Noise**: Stripping repeating headers, footers, copyright notices, and `Return to the Summary Table` links removes thousands of lines of grep pollution across 1,000+ page manuals.
6. **Grep-First Sharded Output + 1-Line-Per-Fact TSV Indexes**: Converting a 1,300-page TRM into one giant `.md` file floods agent searches with uncontextualized line numbers. `chipdoc` shards documents by their bookmark hierarchy (`sections/10-general-purpose-input-output-gpio.md`) and emits flat `.tsv` sidecar indexes (`index/registers.tsv`, `index/tables.tsv`, `index/sections.tsv`) where **every line is self-contained**.

---

## Output Format: The Agent Documentation Bundle

Running `chipdoc convert samples/mspm0c1105_trm.pdf -o output/` produces:

```text
output/mspm0c1105_trm/
├── INDEX.md                        # Master entry point: metadata, stats, full TOC -> section file & page map
├── meta.json                       # Conversion provenance, timing, counts, and sanity score
├── index/
│   ├── sections.tsv                # section_id | level | title | file_path | start_line | page_range
│   ├── tables.tsv                  # table_id | caption | columns | rows | file_path:line | page_range
│   ├── registers.tsv               # peripheral | register | offset | reset | bits | field | access | reset | desc | file:line | page
│   └── figures.tsv                 # figure_id | caption | image_path | file_path:line | page
├── sections/                       # Bookmark-sharded Markdown files with YAML frontmatter
│   ├── 01-architecture.md
│   ├── 02-pmcu.md
│   ├── 10-general-purpose-input-output-gpio.md
│   └── ...
└── assets/
    └── figures/                    # Cropped vector/raster diagrams rendered at 200 DPI
        ├── p007-fig-6-1.png
        └── ...
```

### How an AI Agent Uses the Bundle

1. **Instant Orientation (`Read` `INDEX.md`)**: Reads the full document map, chapter list, and summary statistics in a single lightweight tool call.
2. **One-Shot Register / Bitfield / Spec Lookup (`Grep` `index/registers.tsv` or `sections/`)**:
   ```text
   GPIO	FSUB_0	400h	00000000h	3-0	CHANID	R/W	0h	0 = disconnected; 1-15 = connected to channelID = CHANID	sections/10-general-purpose-input-output-gpio.md:148	564
   ```
   A single grep for `FSUB_0` or `CHANID` returns the peripheral, register, hex offset, reset value, bit range, access type, description, and exact `file:line` pointer in one line.
3. **Targeted Deep Read (`Read` `sections/<chapter>.md` with `offset`/`limit`)**: Reads only the exact section or table lines needed.
4. **Visual Diagram Inspection (`Read` `assets/figures/<fig>.png`)**: Views cropped pinout or block diagram PNGs on demand without wasting tokens on pages of prose.

---

## Quick Start

### Prerequisites

- **Rust 1.85+** (`cargo` and `rustc`)
- **PDFium dynamic library** (`pdfium.dll` on Windows, `libpdfium.so` on Linux, `libpdfium.dylib` on macOS). Run the included helper script to stage `bin/pdfium.dll` automatically:

```powershell
pwsh scripts/fetch_pdfium.ps1
```

### Build

```powershell
cargo build --release
```

### CLI Usage

```powershell
# Convert one or more PDFs into Agent Documentation Bundles
cargo run --release -- convert samples/mspm0c1105_datasheet.pdf samples/mspm0c1105_trm.pdf -o output/

# Also emit a single merged Markdown file alongside the sharded bundle
cargo run --release -- convert samples/mspm0c1105_datasheet.pdf -o output/ --single-file

# Convert only a specific page range (1-based, e.g. GPIO chapter)
cargo run --release -- convert samples/mspm0c1105_trm.pdf -o output/ --pages 557-580

# Inspect PDF metadata, bookmark hierarchy, and page object statistics
cargo run --release -- inspect samples/mspm0c1105_datasheet.pdf

# Search across a generated Agent Bundle (registers, tables, and sections)
cargo run --release -- search output/mspm0c1105_trm "FSUB_0"
```

---

## Development & Verification

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## License

MIT License — see [LICENSE](LICENSE).
