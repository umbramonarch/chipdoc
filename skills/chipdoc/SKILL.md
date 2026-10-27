---
name: chipdoc
description: >-
  Convert, index, and query microcontroller datasheets, Technical Reference Manuals (TRMs),
  errata, schematics/pinouts, and engineering PDFs using the `chipdoc` CLI. Use this skill
  whenever the user asks about hardware registers, bitfields, memory maps, address offsets,
  reset values, pin multiplexing, electrical characteristics, interrupts, DMA triggers, or
  any `.pdf` manual, instead of reading raw PDFs into context or running blind multi-turn greps.
---

# `chipdoc` — Agent Document Intelligence CLI

Never read a raw technical `.pdf` directly into the LLM context window. Use `chipdoc` to
convert PDFs locally at zero LLM token cost into a **Hierarchical Agent Documentation Bundle**
(`INDEX.md` + `index/*.tsv` + `sections/*.md` + `assets/figures/*.png`) and retrieve exact
hardware facts in a **single CLI call**.

## 1. Check Workspace & Ensure Bundles Exist

Before searching or reading documentation, check which bundles are already indexed or ensure a
PDF is converted (cached hits return in **<2 ms** and automatically configure `.claudeignore`
so converted files never invalidate the prompt cache):

```bash
# List all ready bundles and unconverted PDFs in the workspace
chipdoc status

# Convert a PDF only if not already cached (--lazy-figures skips upfront PNG rasterization)
chipdoc ensure path/to/manual.pdf --lazy-figures
```

## 2. Use 1-Turn Domain Queries ("Build the Lever")

Prefer `chipdoc` domain subcommands over multi-turn `Grep` + `Read` loops. Every subcommand
accepts either a `<query>` (searching all bundles in `./output`) or `<bundle_dir> <query>`,
and automatically normalizes C hex (`0x400`, `0x41C00004`) to vendor hex (`400h`, `41C00004h`)
and compound C macro names (`GPIO_FSUB_0`, `GPIO.FSUB_0`):

```bash
# 1. Register, Bitfield, or Hex Offset Lookup (returns offset, reset, and all bitfields in 1 turn)
chipdoc reg FSUB_0
chipdoc reg GPIO_FSUB_0
chipdoc reg 0x41C00004
chipdoc reg output/mspm0c1105_trm BOOTCFG0 --compact

# 2. Table Lookup + Inline Markdown Rendering (memory maps, pinouts, electrical specs)
chipdoc table "Memory Map" --show
chipdoc table output/mspm0c1105_datasheet "Operating Conditions" --show

# 3. Cross-Reference Graph Traversal (links a peripheral, pin, or signal across all chapters)
chipdoc xref ADC0
chipdoc xref PA27

# 4. Exact PDF Page or Chapter Section Extraction
chipdoc page output/mspm0c1105_trm 564
chipdoc section output/mspm0c1105_trm "gpio" --max-lines 120

# 5. Parallel SIMD + Regex Search Across All Bundles
chipdoc search "SWDP_MODE" --compact
```

## 3. On-Demand Visual Diagram Inspection

`chipdoc` suppresses scrambled vector diagram labels from Markdown prose and records figure
bounding boxes in `index/figures.tsv`. Only inspect or render a figure PNG when a task needs a
block diagram, timing chart, or package pinout drawing:

```bash
# List matching figures (and optionally render cropped PNG on demand if --lazy-figures was used)
chipdoc figure "Pin Diagram" --render samples/mspm0c1105_datasheet.pdf
```

Then call `Read` on the specific cropped `assets/figures/<fig>.png` file.
