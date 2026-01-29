#!/usr/bin/env python3
"""Merge Dokka GFM output into a single api.md file with proper hierarchy.

Dokka generates multiple GFM files (one per class/member). This script
uses the directory structure to group members under their parent class,
producing a well-structured markdown file for inclusion in the MkDocs site.

Usage:
    python scripts/merge-dokka-output.py [--input-dir DIR] [--output FILE]

Defaults:
    --input-dir: sdks/kotlin/claim169-core/build/dokka/gfm
    --output:    docs/en/sdk/kotlin/api.md
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

# Package directory name within the Dokka GFM output tree
PACKAGE_NAME = "fr.acn.claim169"

# Logical ordering of classes/interfaces in the output.
# Directories not listed here are appended alphabetically at the end.
CLASS_ORDER: list[str] = [
    "-claim169",
    "-decoder-builder",
    "-encoder-builder",
    "-claim169-data-builder",
    "-cwt-meta-data-builder",
    "-closeable-decode-result",
    "-decoder-configurer",
    "-encoder-configurer",
    "-signature-verifier",
    "-signer",
    "-decryptor",
    "-encryptor",
    "-gender",
    "-marital-status",
    "-photo-format",
    "-cose-algorithm",
    "-verification-status",
    "-verification-result",
]

# Enum entry subdirectories to skip (they add noise).
# These are subdirectories of enum class dirs whose names map to enum values
# or companion objects. We detect them by checking if the subdir only contains
# an index.md file.
SKIP_ENUM_ENTRY_DIRS = True


def find_package_dir(input_dir: Path) -> Path:
    """Locate the package directory inside the Dokka output tree."""
    # Dokka output: <input_dir>/<module-name>/<package-name>/
    candidates = list(input_dir.rglob(PACKAGE_NAME))
    if not candidates:
        print(
            f"ERROR: Package directory '{PACKAGE_NAME}' not found under {input_dir}",
            file=sys.stderr,
        )
        sys.exit(1)
    return candidates[0]


def dir_name_to_class_name(dirname: str) -> str:
    """Convert a Dokka directory name to a PascalCase class name.

    e.g. '-decoder-builder' -> 'DecoderBuilder'
         '-claim169' -> 'Claim169'
    """
    # Strip leading dash
    name = dirname.lstrip("-")
    # Split on dashes and capitalize each part
    parts = name.split("-")
    return "".join(p.capitalize() if not p[0].isdigit() else p for p in parts)


def is_enum_entry_dir(subdir: Path) -> bool:
    """Check if a subdirectory is an enum entry (only contains index.md)."""
    files = list(subdir.iterdir())
    return len(files) == 1 and files[0].name == "index.md"


def demote_headings(content: str, levels: int = 2) -> str:
    """Demote all markdown headings by the given number of levels.

    e.g. with levels=2: # Foo -> ### Foo, ## Bar -> #### Bar
    """

    def _demote(match: re.Match) -> str:
        hashes = match.group(1)
        rest = match.group(2)
        return "#" * (len(hashes) + levels) + rest

    return re.sub(r"^(#{1,6})([ \t])", _demote, content, flags=re.MULTILINE)


def clean_content(content: str) -> str:
    """Clean up Dokka-generated markdown content.

    - Strip breadcrumb navigation lines
    - Remove package-level index links
    """
    lines = content.split("\n")
    cleaned: list[str] = []
    skip_breadcrumbs = True

    for line in lines:
        # Skip breadcrumb navigation lines (e.g., "//[claim169-core](../index.md)/...")
        if skip_breadcrumbs and line.startswith("//"):
            continue
        skip_breadcrumbs = False

        # Skip lines that are just package-level index links
        if re.match(r"^\| \[.*\]\(.*\) \|$", line):
            continue

        cleaned.append(line)

    return "\n".join(cleaned).strip()


def strip_crossfile_links(content: str) -> str:
    """Replace cross-file .md links with plain text.

    Dokka generates links like [Foo](bar.md) or [Foo](../baz/index.md) that
    point to other generated files. Since we merge everything into one file,
    convert these to plain text (just the display text, no link).
    """
    return re.sub(r"\[([^\]]+)\]\([^)]*\.md(?:#[^)]+)?\)", r"\1", content)


# Matches Kotlin declarations, including those with modifiers like
# abstract, open, override, sealed, data before the keyword.
_DECLARATION_RE = re.compile(
    r"^(?:(?:abstract|open|override|sealed|data)\s+)*"
    r"(?:fun|val|var|class|object|enum|interface|constructor)\b"
)


def _wrap_declarations_in_code_fences(content: str) -> str:
    """Wrap standalone Kotlin declarations in ```kotlin fences.

    Skips lines that are already inside code blocks or table rows.
    """
    lines = content.split("\n")
    result: list[str] = []
    in_code_block = False

    for line in lines:
        if line.startswith("```"):
            in_code_block = not in_code_block
            result.append(line)
            continue

        if (
            not in_code_block
            and not line.startswith("|")
            and _DECLARATION_RE.match(line)
        ):
            result.append("```kotlin")
            result.append(line)
            result.append("```")
        else:
            result.append(line)

    return "\n".join(result)


def post_process(content: str) -> str:
    """Clean up Dokka GFM artifacts for readable MkDocs output.

    Handles:
    - [jvm]\\ platform tags
    - Standalone 'jvm' lines in parameter sections
    - @JvmStatic / @JvmName annotation lines
    - Kotlin stdlib links → plain type names
    - HTML entities (&quot; &lt; &gt; &amp;)
    - Consecutive blank lines
    """
    # Remove [jvm]\ platform tags — standalone lines
    content = re.sub(r"^\[jvm\]\\?\s*$", "", content, flags=re.MULTILINE)
    # Remove [jvm]<br> inline in table cells
    content = re.sub(r"\[jvm\](?:<br>)?\\?\s*", "", content)

    # Remove standalone 'jvm' lines (appears under #### Parameters)
    content = re.sub(r"^jvm\s*$", "", content, flags=re.MULTILINE)

    # Decode HTML entities early so regexes below can match plain quotes
    content = content.replace("&quot;", '"')
    content = content.replace("&lt;", "<")
    content = content.replace("&gt;", ">")
    content = content.replace("&amp;", "&")

    # Replace Kotlin stdlib links with plain type names, so that
    # @[JvmStatic](https://kotlinlang.org/...) becomes @`JvmStatic`
    # before JVM annotation removal runs.
    content = re.sub(
        r"\[([A-Za-z0-9_]+)\]\(https://kotlinlang\.org/[^)]+\)", r"`\1`", content
    )

    # Remove @JvmStatic annotations — standalone lines and inline in table cells
    content = re.sub(r"^@`?JvmStatic`?\s*$", "", content, flags=re.MULTILINE)
    content = re.sub(r"@`?JvmStatic`?(?:<br>)?\s*", "", content)

    # Remove @JvmName annotations — standalone lines and inline in table cells
    content = re.sub(
        r'^@`?JvmName`?\(name\s*=\s*"[^"]*"\)\s*$', "", content, flags=re.MULTILINE
    )
    content = re.sub(r'@`?JvmName`?\(name\s*=\s*"[^"]*"\)(?:<br>)?\s*', "", content)

    # Remove empty tables: header + separator with no data rows.
    # Matches tables like: | Name | Summary |\n|---|---|\n\n
    # and also headerless: | | |\n|---|---|\n\n
    content = re.sub(
        r"^\|[^\n]*\|\n\|[-| ]+\|\n(?=\n|\Z)",
        "",
        content,
        flags=re.MULTILINE,
    )

    # Collapse blank lines early so empty heading detection works
    content = re.sub(r"\n{3,}", "\n\n", content)

    # Remove section headings that became empty after table removal
    # (heading followed by nothing but another heading or end of section)
    content = re.sub(
        r"^#{2,6}\s+(?:Constructors|Entries|Types|Functions|Inheritors)\n\n(?=#{2,6}\s|\Z|---)",
        "",
        content,
        flags=re.MULTILINE,
    )

    # Wrap Kotlin declarations in code fences, skipping lines already
    # inside code blocks or table rows.
    content = _wrap_declarations_in_code_fences(content)

    # Collapse runs of 3+ blank lines into 2
    content = re.sub(r"\n{3,}", "\n\n", content)

    return content


def process_class_index(index_path: Path) -> str:
    """Process a class/interface index.md, returning cleaned content with headings demoted."""
    content = index_path.read_text(encoding="utf-8")
    content = clean_content(content)
    # The index.md has a top-level # ClassName heading.
    # We want it at h2, so demote by 1.
    content = demote_headings(content, levels=1)
    return content


def process_member_file(member_path: Path) -> str:
    """Process a member file (method/property), returning cleaned content with headings demoted."""
    content = member_path.read_text(encoding="utf-8")
    content = clean_content(content)
    # Member files have # memberName heading.
    # We want members at h3, so demote by 2.
    content = demote_headings(content, levels=2)
    return content


def process_class_dir(class_dir: Path) -> str:
    """Process an entire class directory into a structured markdown section.

    Returns a section with:
    - ## ClassName (from index.md)
    - ### memberName (from each member .md file)
    """
    sections: list[str] = []

    # Process the class index file
    index_path = class_dir / "index.md"
    if index_path.exists():
        sections.append(process_class_index(index_path))

    # Process member files (skip index.md, skip enum entry subdirs)
    member_files = sorted(
        f
        for f in class_dir.iterdir()
        if f.is_file() and f.suffix == ".md" and f.name != "index.md"
    )

    for member_file in member_files:
        content = process_member_file(member_file)
        if content.strip():
            sections.append(content)

    # Process nested type subdirs that are NOT enum entries
    # (e.g., -companion for companion objects, -invalid/-valid for sealed classes)
    subdirs = sorted(d for d in class_dir.iterdir() if d.is_dir())

    for subdir in subdirs:
        if SKIP_ENUM_ENTRY_DIRS and is_enum_entry_dir(subdir):
            continue

        # For companion objects, include their member files but skip the index
        subdir_files = sorted(
            f
            for f in subdir.iterdir()
            if f.is_file() and f.suffix == ".md" and f.name != "index.md"
        )
        for member_file in subdir_files:
            content = process_member_file(member_file)
            if content.strip():
                sections.append(content)

    return "\n\n".join(sections)


def process_top_level_files(pkg_dir: Path) -> str:
    """Process top-level .md files (package-level functions) into a section."""
    top_files = sorted(
        f
        for f in pkg_dir.iterdir()
        if f.is_file() and f.suffix == ".md" and f.name != "index.md"
    )

    if not top_files:
        return ""

    sections: list[str] = ["## Top-Level Functions"]

    for f in top_files:
        content = f.read_text(encoding="utf-8")
        content = clean_content(content)
        # Demote headings: # funcName -> ### funcName
        content = demote_headings(content, levels=2)
        if content.strip():
            sections.append(content)

    return "\n\n".join(sections)


def build_document(pkg_dir: Path) -> str:
    """Build the full API reference document from the Dokka package directory."""
    sections: list[str] = []

    # Collect class directories
    class_dirs = [d for d in pkg_dir.iterdir() if d.is_dir() and d.name.startswith("-")]

    # Sort by the defined order; unlisted classes go at the end alphabetically
    order_map = {name: i for i, name in enumerate(CLASS_ORDER)}

    def sort_key(d: Path) -> tuple[int, str]:
        return (order_map.get(d.name, len(CLASS_ORDER)), d.name)

    class_dirs.sort(key=sort_key)

    # Process each class directory
    for class_dir in class_dirs:
        section = process_class_dir(class_dir)
        if section.strip():
            sections.append(section)

    # Process top-level functions
    top_level = process_top_level_files(pkg_dir)
    if top_level.strip():
        sections.append(top_level)

    return "\n\n---\n\n".join(sections)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Merge Dokka GFM output into single api.md"
    )
    parser.add_argument(
        "--input-dir",
        type=Path,
        default=Path("sdks/kotlin/claim169-core/build/dokka/gfm"),
        help="Dokka GFM output directory",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path("docs/en/sdk/kotlin/api.md"),
        help="Output api.md file path",
    )
    args = parser.parse_args()

    pkg_dir = find_package_dir(args.input_dir)
    merged = build_document(pkg_dir)

    header = (
        "# API Reference\n\n"
        "Complete API documentation for the claim169 Kotlin SDK, "
        "auto-generated from source code using [Dokka](https://kotl.in/dokka).\n\n"
    )

    merged = strip_crossfile_links(merged)
    merged = post_process(merged)
    output = header + merged
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(output + "\n", encoding="utf-8")

    print(f"Kotlin API docs generated at {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
