# Changelog

All notable changes to this project are listed here. Versions follow
[Semantic Versioning](https://semver.org/), and the version lives in
`.claude-plugin/plugin.json` and `Cargo.toml`.

## [Unreleased]

## [1.1.0] - 2026-10-27

### Added

- **Expanded Universal Register & Bitfield Extraction (`registers.rs`, `stitcher.rs`, `emitter.rs`)**:
  - Support for arrayed and formula-offset inline register headings (`REG[y] (Offset = 41C00024h + formula)`), 2-line wrapped register headings with alternate modes (`[alternate] (PERIPH_REG)`), kerning spaces inside parenthesized acronyms (`(PERIPH_REG1 0)` -> `PERIPH_REG10`), `Address:` metadata lines, unnumbered bold register headings, numbered plain register headings, and prefixed/binary reset values (`System reset value:`, `Backup domain reset value:`, `POR:`, `0b...`).
  - Support for non-contiguous bit lists (`Bits 24, 14:12`), `+` arithmetic in field names (`EXTIm+3[7:0]`), space-delimited uppercase prose bitfields without colons (`Bits 31:16 REV_ID[15:0] Revision identifier`), and context-aware `R/W` vs `RO` access inference that avoids false-positive `RO` on conditional `"read-only when..."` clauses.
  - Long uppercase identifier line-wrap joining inside narrow table columns (`FACTORYRESETCMDAC\nCESS` -> `FACTORYRESETCMDACCESS`).
  - Peripheral context tracking across chapter headings, register acronyms, slash-separated peripheral groups, and multi-page register definitions while ignoring functional subheadings.
- **Geometric Table, Stitching & Figure Improvements (`tables.rs`, `stitcher.rs`, `figures.rs`)**:
  - Break detection between vertically separated table clusters when separated by captions, section/register headings, 16-/32-bit bit-number headers, callout headers (`Note`), or full-width body paragraphs.
  - Sub-column track discovery inside wide unruled table columns (`>110 pt`) such as multi-package pinout tables.
  - Lookahead cross-page table stitching across intervening footnote/page-number tables (`i + 2`) and minor column-count differences (`<= 1` empty margin column).
  - Uncaptioned vector diagram rejection in `tables.rs` and strict caption/prose boundary detection in `figures.rs`.
- **Cleaner, Bookmark & Sharding Refinements (`cleaner.rs`, `pdf_loader.rs`, `emitter.rs`, `sanity.rs`)**:
  - Inline HTML/XML markup tag stripping (`<Sup>`, `</Sup>`, `<sub>`, etc.) in body text, PDF metadata, and bookmarks.
  - Filtering of `Table N.` and `Figure N.` caption bookmarks from chapter sharding while preserving `Table of Contents`.
  - Shared-page section boundary routing in `stitcher.rs` so trailing subsections on a boundary page stay in their parent chapter before the next top-level heading.
  - Alternating odd/even running header/footer stripping, whole-line margin eviction, `N/M` page counter stripping, superscript attachment (`I2C`, `®`, `™`), and acronym/cross-reference boundary spacing.
  - Automatic cleanup of stale `.md` files in `sections/` during bundle emission.
- **Hardware Cross-Reference Graph & 1-Turn Query Engine (`xref.rs`, `query.rs`, `main.rs`)**:
  - Full-row scanning (`take(12)`) for pin names and memory-map addresses in wide multi-package tables, plus peripheral sanitization to prevent `(continued)` table artifacts.
  - Multi-page `RegCard` merging, template-to-concrete peripheral/register matching (`GPIOA_MODER` <-> `GPIOx_MODER`, `USART1_CR1` <-> `USART_CR1`, `DMA1_CCR1` <-> `DMA_CCRx`), and underscore bitfield fallback (`SWDP_MODE`, `OBL_LAUNCH`).
  - `chipdoc page <N>` multi-bundle lookup when `<bundle>` is omitted, dotted sub-section lookup (`chipdoc section 10.3`), and exact word-boundary pin/symbol matching in `chipdoc xref`.

## [1.0.0] - 2026-10-26

### Added

- **Core PDF-to-Agent Extraction Engine (`crates/chipdoc-core`)**:
  - PDFium-backed document loader (`pdf_loader.rs`) with fast FNV-1a file fingerprinting, bookmark (`TOC`) hierarchy extraction, positioned text spans, vector path segments, and raster images.
  - Parallel running header/footer and boilerplate stripper (`cleaner.rs`) with Unicode and subscript normalization (`–0.3` -> `-0.3`, `℃` -> `°C`, `VIInput` -> `VI Input`).
  - Vector and raster figure region detector (`figures.rs`) supporting both top-caption and bottom-caption figure conventions, suppressing scrambled diagram labels from Markdown prose, and rendering cropped figure PNGs (`assets/figures/`) with parallel fast PNG encoding or `--lazy-figures`.
  - Geometric vector-grid and coordinate-aligned table engine (`tables.rs` & `stitcher.rs`) with multi-level header flattening, mid-identifier wrap repair (`GEN_EVENT\n0` -> `GEN_EVENT0`), navigation link column removal (`Go`), diagram-grid rejection, and cross-page `(continued)` table stitching.
  - Multi-format register and bitfield extractor (`registers.rs`) supporting both inline/tabular register definitions (`Offset = ...` + `Field Descriptions` tables) and parenthesized/prose register definitions (`Address offset: 0x...` + `Bits X:Y FIELD:` blocks), with visual 16-/32-bit bit-box deduplication (`index/registers.tsv`).
  - Hardware Cross-Reference Graph builder (`xref.rs`) emitting `index/xref.tsv` to link peripherals, registers, bitfields, pin/memory-map table rows, figures, and chapters across the document.
  - Hierarchical Agent Documentation Bundle emitter (`emitter.rs`) writing `INDEX.md`, `sections/*.md`, `index/*.tsv`, `meta.json`, `--single-file`, and `.claudeignore` prompt-cache protection.
- **1-Turn Agent Document Intelligence CLI (`crates/chipdoc-cli` & `query.rs`)**:
  - `chipdoc status [--compact]`: Scans the workspace for ready bundles and unconverted `.pdf` files.
  - `chipdoc ensure <pdf...> [--lazy-figures]`: Converts PDFs with `<2ms` incremental cache hits when unchanged.
  - `chipdoc reg <query> [--compact]`: 1-turn register and bitfield lookup with C/vendor hex normalization (`0x41C00004` <-> `41C00004h`) and peripheral-prefix splitting (`GPIO_FSUB_0` <-> `GPIO.FSUB_0`).
  - `chipdoc table <query> [--show]`: Searches table captions/columns and prints reconstructed Markdown tables inline.
  - `chipdoc xref <symbol>` (alias `chipdoc explain`): Traverses the Hardware Cross-Reference Graph across all bundles.
  - `chipdoc page <bundle> <pages>` and `chipdoc section [query]`: Extracts Markdown slices by original 1-based PDF page number or chapter shard.
  - `chipdoc figure [query] [--render <pdf>]`: Lists detected figures and renders cropped PNGs on demand.
  - `chipdoc search <pattern> [--compact]`: Parallel SIMD (`memchr`) + Regex search across all bundles in `./output`.
  - `chipdoc convert` and `chipdoc inspect`.
- **Hybrid Claude Code Plugin (`.claude-plugin/`, `hooks/`, `scripts/`, `skills/chipdoc/`)**:
  - `.claude-plugin/plugin.json` and `.claude-plugin/marketplace.json` manifests.
  - `SessionStart` hook (`scripts/session_start_hook.py`) injecting workspace bundle and PDF status into session context.
  - `PreToolUse` hook (`scripts/pre_read_pdf_hook.py`) blocking raw `.pdf` `Read` calls and steering agents to `chipdoc`.
  - `/chipdoc` skill (`skills/chipdoc/SKILL.md`) teaching 1-turn CLI retrieval workflows.

[Unreleased]: https://github.com/umbramonarch/chipdoc/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/umbramonarch/chipdoc/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/umbramonarch/chipdoc/releases/tag/v1.0.0
