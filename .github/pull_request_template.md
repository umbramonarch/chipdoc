## Summary

<!-- Describe what this pull request changes and why. -->

## Related Issue

<!-- Link the related issue or task (e.g., Closes #123). -->

## Test Plan

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] `CHANGELOG.md` updated under `[Unreleased]` (for user-visible changes)

## Extraction Quality Checklist

- [ ] Heuristics remain 100% vendor-neutral (no hardcoded chip or vendor names)
- [ ] No raw sample PDFs or generated output directories are committed
