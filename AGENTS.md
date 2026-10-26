# Repository Guidelines (`chipdoc`)

## Project Structure & Module Organization

`chipdoc` converts technical and embedded-systems PDFs (microcontroller datasheets, Technical Reference Manuals, errata, and application notes) into **Hierarchical Agent Documentation Bundles** (`INDEX.md` + chapter `.md` files + flat `.tsv` grep/xref indexes + cropped figure `.png` assets) and provides a 1-turn **Agent Document Intelligence CLI** and **Hybrid Claude Code Plugin**.

- `Cargo.toml`: Workspace manifest defining shared dependencies, lints, and release profiles.
- `crates/chipdoc-core/`: Core extraction, geometry, table reconstruction, register indexing, cross-reference graph, query engine, and bundle emission library.
  - `src/model.rs`: Canonical domain structures (`BBox`, `TextSpan`, `TocEntry`, `ExtractedTable`, `RegisterBitfield`, `FigureRegion`, `XrefEntry`, `ConvertConfig`).
  - `src/pdf_loader.rs`: PDFium runtime binding, fast FNV-1a file fingerprinting, metadata & outline (`TOC`) extraction, page text/path/image extraction, and parallel/on-demand figure rendering.
  - `src/cleaner.rs`: Parallel running header/footer removal, boilerplate stripping, Unicode normalization (`U+2013` -> `-`, `℃` -> `°C`), and subscript kerning spacing (`VIInput` -> `VI Input`).
  - `src/figures.rs`: Figure caption detection, vector/raster bounding-box clustering, diagram text-span suppression, and cropped PNG region calculation.
  - `src/tables.rs`: Geometric vector-grid and coordinate-aligned table reconstruction, multi-row header flattening, mid-identifier wrap joining, and navigation column filtering.
  - `src/registers.rs`: Visual 16-/32-bit register bit-box deduplication, grep-anchored register header formatting, and `registers.tsv` extraction.
  - `src/stitcher.rs`: Cross-page `(continued)` table stitching and bookmark-driven section sharding.
  - `src/xref.rs`: Hardware Cross-Reference Graph builder (`index/xref.tsv`) linking peripherals, registers, bitfields, table rows/pins, figures, and chapters.
  - `src/query.rs`: Multi-bundle discovery, hardware hex/symbol normalizer (`0x400` <-> `400h`, `GPIO_FSUB_0` <-> `GPIO.FSUB_0`), 1-turn domain queries (`reg`, `table`, `section`, `page`, `xref`, `figure`), and parallel SIMD (`memchr` + `rayon`) search.
  - `src/emitter.rs`: Output writer for `INDEX.md`, `sections/*.md`, `index/*.tsv`, `meta.json`, `.claudeignore`, and `--single-file`.
  - `src/sanity.rs`: Conversion sanity scoring and token-efficiency diagnostics.
- `crates/chipdoc-cli/`: Command-line interface binary (`chipdoc`) implementing `convert`, `ensure`, `status`, `inspect`, `reg`, `table`, `section`, `page`, `xref`, `figure`, and `search`.
- `.claude-plugin/`, `hooks/`, `scripts/`, `skills/chipdoc/`: Hybrid Claude Code plugin (`SessionStart` bundle discovery hook, `PreToolUse` raw PDF read guard, and `SKILL.md`).
- `samples/`: Local PDF test corpus (`samples/*.pdf` is gitignored; `samples/README.md` is tracked).
- `bin/`: Local runtime location for `pdfium.dll` / `libpdfium.so` / `libpdfium.dylib` (gitignored).
- `scripts/fetch_pdfium.ps1`: PowerShell helper to download or copy the platform PDFium dynamic library into `bin/`.

## Build, Test, and Development Commands

- `pwsh scripts/fetch_pdfium.ps1`: Ensure `bin/pdfium.dll` is present locally.
- `cargo build --release`: Build optimized `chipdoc` binary (`target/release/chipdoc.exe`).
- `cargo test --workspace`: Run all unit and integration tests across the workspace.
- `cargo fmt --all -- --check`: Check Rust formatting.
- `cargo clippy --workspace --all-targets -- -D warnings`: Run strict static analysis with zero warnings permitted.
- `cargo run --release -p chipdoc-cli -- ensure samples/mspm0c1105_datasheet.pdf samples/mspm0c1105_trm.pdf -o output/`: Ensure PDFs are converted and cached.
- `cargo run --release -p chipdoc-cli -- status`: Show ready bundles and unconverted PDFs in the workspace.
- `cargo run --release -p chipdoc-cli -- reg 0x41C00004`: Query register by C hex offset, vendor hex offset, or peripheral/register name.
- `cargo run --release -p chipdoc-cli -- table "Memory Map" --show`: Search tables and render inline Markdown.
- `cargo run --release -p chipdoc-cli -- xref PA27`: Query the Hardware Cross-Reference Graph across all bundles.

## Coding Style & Naming Conventions

- **Rust Edition 2021+**: Follow `rustfmt` defaults (4-space indentation, `snake_case` functions/modules, `PascalCase` types/traits, `SCREAMING_SNAKE_CASE` constants).
- **Vendor-Neutral Rules**: Keep all extraction heuristics 100% vendor-neutral. Never hardcode specific chip part numbers or vendor strings to pass a single test document; rely on structural patterns (vector geometry, bookmark hierarchy, bit-range patterns `31..0`, hex offset syntax `0x...` / `...h`, caption conventions `Figure X-Y` / `Table X-Y`).
- **Error Handling**: Use typed errors (`thiserror`) in `chipdoc-core` and contextual reporting (`anyhow`) in `chipdoc-cli`. Never `unwrap()` untrusted PDF content streams when graceful fallback is possible.

## Commit & Pull Request Guidelines

- **Branch workflow**: Work on dedicated feature or fix branches (`feat/<topic>`, `fix/<topic>`, `docs/<topic>`, `chore/<topic>`) and merge to `main`.
- **Verification before commit**: Every commit must pass `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.
- Update `CHANGELOG.md` under `[Unreleased]` for user-visible features or fixes.
