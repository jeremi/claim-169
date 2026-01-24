#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import sys


def list_md_files(root: Path) -> set[Path]:
    return {
        p.relative_to(root)
        for p in root.rglob("*.md")
        if p.is_file() and "README.md" not in p.parts
    }


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    docs_root = repo_root / "docs"

    en_root = docs_root / "en"
    if not en_root.exists():
        print("ERROR: docs/en does not exist", file=sys.stderr)
        return 2

    en_files = list_md_files(en_root)
    if not en_files:
        print("ERROR: no .md files found under docs/en", file=sys.stderr)
        return 2

    locales = ["fr"]
    missing: dict[str, list[str]] = {loc: [] for loc in locales}

    for loc in locales:
        loc_root = docs_root / loc
        if not loc_root.exists():
            missing[loc].extend([str(p) for p in sorted(en_files)])
            continue

        loc_files = list_md_files(loc_root)
        for rel in sorted(en_files):
            if rel not in loc_files:
                missing[loc].append(str(rel))

    any_missing = False
    for loc in locales:
        if missing[loc]:
            any_missing = True
            print(f"Missing docs/{loc}/ pages (expected to match docs/en/):")
            for rel in missing[loc]:
                print(f"  - {rel}")

    if any_missing:
        print(
            "\nTip: create empty placeholder pages if you want intentional EN fallback,\n"
            "so you can track translation debt explicitly.",
            file=sys.stderr,
        )
        return 1

    print("OK: FR docs match EN page set.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
