# `chipdoc` — Agent Document Intelligence CLI & Plugin for Technical PDFs

`chipdoc` is a fast, vendor-neutral **Rust CLI, library, and Claude Code plugin** that turns microcontroller datasheets, Technical Reference Manuals (TRMs), errata, and engineering PDFs into a queryable **Hierarchical Agent Documentation Bundle** and **Hardware Cross-Reference Graph** (`INDEX.md` + `index/*.tsv` + `sections/*.md` + cropped `assets/figures/*.png`).

Instead of burning 30,000+ vision tokens reading raw PDF pages or running 5 sequential `Grep` + `Read` turns, an AI coding agent uses `chipdoc` to resolve registers, bitfields, hex offsets, memory maps, pinouts, and cross-chapter hardware dependencies in **a single deterministic CLI call (~150 tokens)**.

---

## Why `chipdoc`?

General-purpose PDF-to-Markdown tools (`markitdown`, `docling`, `pymupdf4llm`) struggle with embedded-systems documentation for six concrete reasons:

1. **Vector Diagrams Become Scrambled Text**: Pinout diagrams, block diagrams, and timing charts in datasheets are drawn using **PDF vector paths + floating text labels**, not bitmap images. Naive extractors dump hundreds of floating pin labels (`PA0`, `VDD`, `36`, `PB17`) into the middle of Markdown prose in arbitrary Y-coordinate order. `chipdoc` detects figure bounding boxes, **suppresses floating diagram text from polluting Markdown**, and **renders cropped figure PNGs** to `assets/figures/` (either upfront or lazily on demand).
2. **Redundant Visual Register Bit-Boxes**: Every register in a TRM includes a 16- or 32-bit visual bit-grid figure immediately above a `Field Descriptions` table with the exact same information. `chipdoc` condenses the noisy visual bit-grid into a 1-line bit-map summary and preserves the clean Markdown table, cutting register-page token usage by **30–40%**.
3. **Complex Multi-Header & Multi-Page Tables**: Electrical characteristic tables use 2-level merged headers (`-40°C / 25°C / 85°C` $\times$ `TYP / MAX`) and mode divider rows (`RUN Mode`, `SLEEP Mode`), while pinout and register tables span many pages with `(continued)` headers and mid-identifier line wraps (`GEN_EVENT\n0`). `chipdoc` reconstructs cell grids from PDF vector geometry, flattens multi-row headers, repairs wrapped identifiers, drops hyperlink filler columns (`Section: Go`), and stitches `(continued)` tables across page breaks.
4. **Unicode & Subscript Kerning Traps**: Datasheets frequently use Unicode en-dashes for negative numbers (`–0.3 V`), single-glyph Celsius symbols (`℃`), and tight subscript kerning that fuses symbols into adjacent words (`VIInput voltage`), causing exact `grep` queries to miss. `chipdoc` normalizes these deterministically (`-0.3 V`, `°C`, `VI Input voltage`).
5. **Hardware Syntax Normalization**: Firmware engineers and agents write `0x41C00004` or `GPIO_FSUB_0`, while datasheets write `41C00004h` or split `GPIO` and `FSUB_0` across columns. `chipdoc reg` and `chipdoc xref` normalize C hex (`0x400` $\leftrightarrow$ `400h`) and peripheral prefixes (`GPIO.FSUB_0` / `GPIO_FSUB_0` / `GPIO->FSUB_0`) automatically.
6. **Hardware Cross-Reference Graph (`index/xref.tsv`)**: Like Graphify for codebases, `chipdoc` precomputes a cross-reference graph linking peripherals, registers, bitfields, pins, memory map rows, and figures across all chapters with zero LLM API calls.

---

## Output Format: The Agent Documentation Bundle

Running `chipdoc convert samples/mspm0c1105_trm.pdf -o output/` (or `chipdoc ensure samples/mspm0c1105_trm.pdf`) produces:

```text
output/mspm0c1105_trm/
├── INDEX.md                        # Master entry point: metadata, stats, full TOC -> section file & page map
├── meta.json                       # Conversion provenance, FNV-1a source fingerprint, timing, and sanity score
├── index/
│   ├── sections.tsv                # section_id | title | file_path | lines | pages | tables | registers | figures
│   ├── tables.tsv                  # table_id | caption | columns | rows | location | pages
│   ├── registers.tsv               # peripheral | register | offset | reset | bits | field | access | field_reset | description | location | page
│   ├── figures.tsv                 # figure_id | caption | image_path | location | page | bbox
│   └── xref.tsv                    # symbol | kind | peripheral | summary | location | page
├── sections/                       # Bookmark-sharded Markdown files with YAML frontmatter
│   ├── 01-architecture.md
│   ├── 02-pmcu.md
│   ├── 12-10-general-purpose-input-output-gpio.md
│   └── ...
└── assets/
    └── figures/                    # Cropped vector/raster diagrams rendered at 200 DPI
        ├── p007-fig-6-1.png
        └── ...
```

`chipdoc` also adds `output/` to `.claudeignore` automatically so generating or updating bundles never invalidates Claude Code's prompt cache.

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

### 1-Turn Agent CLI Usage

```powershell
# 1. Check workspace status (ready bundles + unconverted PDFs)
chipdoc status

# 2. Ensure PDFs are converted (returns in ~1ms on cache hit; --lazy-figures defers PNG rendering)
chipdoc ensure samples/mspm0c1105_datasheet.pdf samples/mspm0c1105_trm.pdf --lazy-figures

# 3. Look up registers & bitfields by name, peripheral.reg, or C/vendor hex offset
chipdoc reg GPIO_FSUB_0
chipdoc reg 0x41C00004 --compact

# 4. Search tables and print the reconstructed Markdown table inline
chipdoc table "Top Level Memory Map" --show

# 5. Traverse the Hardware Cross-Reference Graph across chapters, registers, pins, and tables
chipdoc xref PA27
chipdoc xref ADC0

# 6. Extract Markdown for an exact PDF page or chapter section
chipdoc page output/mspm0c1105_trm 564
chipdoc section output/mspm0c1105_trm "gpio" --max-lines 100

# 7. List figures and render a cropped PNG on demand
chipdoc figure "Pin Diagram" --render samples/mspm0c1105_datasheet.pdf

# 8. Parallel SIMD + Regex search across all bundles in output/
chipdoc search "SWDP_MODE" --compact
```

---

## Hybrid Claude Code Plugin (`CLI + Hooks + Thin Skill`)

`chipdoc` ships as a complete Claude Code plugin:

- **`.claude-plugin/plugin.json` & `marketplace.json`**: Self-hosted plugin and marketplace manifests.
- **`hooks/hooks.json`**:
  - **`SessionStart` hook (`scripts/session_start_hook.py`)**: Scans the workspace in <10 ms and injects a compact summary of ready `chipdoc` bundles and unconverted PDFs into session context.
  - **`PreToolUse` hook (`scripts/pre_read_pdf_hook.py`)**: Intercepts direct `Read` calls on raw `.pdf` files and redirects the agent to `chipdoc ensure` + `chipdoc reg / table / xref / page`.
- **`skills/chipdoc/SKILL.md`**: Concise skill teaching the agent the 1-turn `chipdoc` CLI commands.

### Install in Claude Code

```bash
claude plugin marketplace add umbramonarch/chipdoc
claude plugin install chipdoc@chipdoc
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
