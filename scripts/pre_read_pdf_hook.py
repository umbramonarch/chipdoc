#!/usr/bin/env python3
"""PreToolUse hook for the chipdoc Claude Code plugin.

Intercepts direct `Read` tool calls on raw `.pdf` files and steers the agent to use
`chipdoc` (which converts the PDF locally with zero LLM tokens and exposes 1-turn
register, table, page, and cross-reference queries).

Set `CHIPDOC_ALLOW_RAW_PDF=1` in the environment to bypass this guard when raw PDF
inspection is explicitly required.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import sys


def main() -> int:
    if os.environ.get("CHIPDOC_ALLOW_RAW_PDF") == "1":
        return 0

    try:
        raw_stdin = sys.stdin.read()
        if not raw_stdin.strip():
            return 0
        event = json.loads(raw_stdin)
    except Exception:
        return 0

    tool_name = event.get("tool_name", "")
    if tool_name != "Read":
        return 0

    tool_input = event.get("tool_input", {}) or {}
    file_path_str = tool_input.get("file_path") or tool_input.get("path") or ""
    if not isinstance(file_path_str, str) or not file_path_str.lower().endswith(".pdf"):
        return 0

    pdf_path = Path(file_path_str)
    stem = pdf_path.stem
    project_dir = Path(os.environ.get("CLAUDE_PROJECT_DIR", ".")).resolve()
    bundle_dir = project_dir / "output" / stem

    if (bundle_dir / "meta.json").is_file() and (bundle_dir / "INDEX.md").is_file():
        reason = (
            f"[chipdoc] Blocked raw PDF Read on '{file_path_str}': an indexed Agent Documentation Bundle "
            f"already exists at 'output/{stem}/'. Reading raw PDFs wastes thousands of vision tokens. "
            f"Query the bundle in 1 turn using:\n"
            f"  - `chipdoc reg output/{stem} <REG_OR_0xOFFSET>`\n"
            f"  - `chipdoc table output/{stem} \"<KEYWORD>\" --show`\n"
            f"  - `chipdoc xref output/{stem} <SYMBOL>`\n"
            f"  - `chipdoc page output/{stem} <PAGE_NUM>`\n"
            f"  - or Read `output/{stem}/INDEX.md`."
        )
    else:
        reason = (
            f"[chipdoc] Blocked raw PDF Read on '{file_path_str}': reading raw technical PDFs into context "
            f"wastes thousands of tokens and scrambles vector diagrams/tables. First convert it locally in "
            f"one command:\n"
            f"  `chipdoc ensure \"{file_path_str}\" --lazy-figures`\n"
            f"Then query `output/{stem}/` with `chipdoc reg`, `chipdoc table --show`, `chipdoc xref`, or `chipdoc page`."
        )

    response = {
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": reason,
        }
    }
    sys.stdout.write(json.dumps(response) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
