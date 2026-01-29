#!/usr/bin/env bash
# Generate TypeScript API documentation using typedoc.
#
# Runs typedoc to produce markdown files, then merges them into a single
# api.md file under docs/en/sdk/typescript/.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TS_SDK_DIR="${REPO_ROOT}/sdks/typescript"
OUT_DIR="${TS_SDK_DIR}/../../docs/en/sdk/typescript/api-gen"
TARGET="${REPO_ROOT}/docs/en/sdk/typescript/api.md"

cd "${TS_SDK_DIR}"

# Run typedoc to generate markdown
npx typedoc

# Merge generated files into a single api.md
{
  echo "# API Reference"
  echo ""
  echo "Complete API documentation for the claim169 TypeScript SDK, auto-generated from source code."
  echo ""

  # Concatenate all generated markdown files in sorted order
  for f in "${OUT_DIR}"/*.md; do
    if [ -f "$f" ]; then
      echo ""
      echo "---"
      echo ""
      cat "$f"
    fi
  done
} > "${TARGET}.tmp"

# Convert cross-file links to plain text or same-page anchors.
# typedoc generates links like [Foo](claim169.types.Interface.Foo.md) or
# [Foo](claim169.types.Interface.Foo.md#method). Since we merged everything
# into one file, strip the .md file references entirely — replace with the
# display text only (no link). This avoids broken links in the merged output.
python3 -c "
import re, sys

content = open('${TARGET}.tmp').read()

# Replace [text](anything.md) and [text](anything.md#anchor) with just text
content = re.sub(r'\[([^\]]+)\]\([^)]*\.md(?:#[^)]+)?\)', r'\1', content)

sys.stdout.write(content)
" > "${TARGET}"

rm -f "${TARGET}.tmp"

# Clean up generated directory
rm -rf "${OUT_DIR}"

echo "TypeScript API docs generated at ${TARGET}"
