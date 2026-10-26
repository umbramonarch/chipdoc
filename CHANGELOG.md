# Changelog

All notable changes to `chipdoc` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-26

### Added
- Initial Cargo workspace (`chipdoc-core` library and `chipdoc-cli` binary `chipdoc`).
- PDFium-backed document loader extracting bookmarks (`TOC`), text spans with bounding boxes and font metrics, vector paths, and raster images.
- Running header/footer and boilerplate stripper with Unicode/subscript normalization (`–0.3` -> `-0.3`, `℃` -> `°C`, `VIInput` -> `VI Input`).
- Vector and raster figure region detector (`Figure X-Y` captions + path/image bounding box clustering) that suppresses scrambled diagram labels in Markdown prose and renders cropped figure PNGs to `assets/figures/`.
- Geometric vector-grid and whitespace table engine with multi-level header flattening, mid-identifier wrap repair (`GEN_EVENT\n0` -> `GEN_EVENT0`), navigation link column removal (`Go`), and cross-page `(continued)` table stitching.
- Visual 16-/32-bit register bit-box deduplicator and register/bitfield parser emitting 1-line-per-bitfield grep indexes (`index/registers.tsv`).
- Hierarchical Agent Documentation Bundle emitter (`INDEX.md`, `sections/*.md`, `index/*.tsv`, `meta.json`, and `--single-file` mode).
- CLI subcommands: `chipdoc convert`, `chipdoc inspect`, and `chipdoc search`.
