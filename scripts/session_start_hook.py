#!/usr/bin/env python3
"""SessionStart hook for the chipdoc Claude Code plugin.

Scans the project directory in <10ms for converted chipdoc bundles (`output/*/meta.json`)
and unconverted `.pdf` files, injecting a compact orientation summary into the session
context so the agent immediately knows which datasheets/TRMs are available.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import sys

IGNORED_DIRS = {".git", "target", "node_modules", "bin", "output", ".claude", ".factory"}


def find_bundles(root: Path) -> list[dict]:
    bundles: list[dict] = []
    candidates = [root, root / "output"]
    seen: set[Path] = set()
    for base in candidates:
        if not base.is_dir():
            continue
        try:
            for child in sorted(base.iterdir()):
                if not child.is_dir() or child in seen:
                    continue
                meta_path = child / "meta.json"
                index_path = child / "INDEX.md"
                if meta_path.is_file() and index_path.is_file():
                    seen.add(child)
                    try:
                        data = json.loads(meta_path.read_text(encoding="utf-8"))
                        meta = data.get("metadata", {})
                        sanity = data.get("sanity", {})
                        bundles.append(
                            {
                                "stem": meta.get("doc_stem", child.name),
                                "path": child.relative_to(root).as_posix(),
                                "pages": meta.get("converted_pages", 0),
                                "regs": sanity.get("registers_indexed", 0),
                                "tables": sanity.get("tables_extracted", 0),
                                "xrefs": sanity.get("xrefs_indexed", 0),
                            }
                        )
                    except Exception:
                        continue
        except OSError:
            continue
    return bundles


def find_pdfs(root: Path, converted_stems: set[str], max_depth: int = 3) -> list[str]:
    pdfs: list[str] = []

    def walk(curr: Path, depth: int) -> None:
        if depth > max_depth:
            return
        try:
            for entry in sorted(curr.iterdir()):
                if entry.is_dir():
                    if entry.name.startswith(".") or entry.name in IGNORED_DIRS:
                        continue
                    walk(entry, depth + 1)
                elif entry.suffix.lower() == ".pdf" and entry.stem not in converted_stems:
                    pdfs.append(entry.relative_to(root).as_posix())
        except OSError:
            return

    walk(root, 0)
    return pdfs


def main() -> int:
    project_dir = Path(os.environ.get("CLAUDE_PROJECT_DIR", ".")).resolve()
    bundles = find_bundles(project_dir)
    converted_stems = {b["stem"] for b in bundles}
    unconverted = find_pdfs(project_dir, converted_stems)

    if not bundles and not unconverted:
        return 0

    lines: list[str] = ["[chipdoc] Hardware & Technical Document Intelligence active:"]
    if bundles:
        summary = ", ".join(
            f"`{b['path']}` ({b['pages']}pp, {b['regs']} regs, {b['tables']} tables)"
            for b in bundles
        )
        lines.append(f"- Indexed bundles ready: {summary}")
    if unconverted:
        pdf_list = ", ".join(f"`{p}`" for p in unconverted[:8])
        lines.append(
            f"- Unconverted PDFs detected: {pdf_list} (run `chipdoc ensure <pdf> --lazy-figures` before querying)"
        )
    lines.append(
        "- Use 1-turn CLI levers instead of reading raw PDFs: "
        "`chipdoc reg <name|0xHEX>`, `chipdoc table <query> --show`, "
        "`chipdoc xref <symbol>`, `chipdoc page <N>`, `chipdoc search <pattern>`."
    )

    payload = {
        "hookSpecificOutput": {
            "hookEventName": "SessionStart",
            "additionalContext": "\n".join(lines),
        }
    }
    sys.stdout.write(json.dumps(payload) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
