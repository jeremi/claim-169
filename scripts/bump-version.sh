#!/usr/bin/env bash
# Synchronizes version across all packages in the monorepo
#
# Usage: ./scripts/bump-version.sh <version>
# Example: ./scripts/bump-version.sh 0.2.0
#          ./scripts/bump-version.sh 0.2.0-alpha

set -euo pipefail

VERSION="${1:?Usage: bump-version.sh <version>}"

# Validate semver format (with optional pre-release)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    echo "Error: Invalid version format: $VERSION"
    echo "Expected: X.Y.Z or X.Y.Z-prerelease (e.g., 1.0.0, 1.0.0-alpha, 1.0.0-beta.1)"
    exit 1
fi

echo "Bumping all packages to version $VERSION"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

# 1. Update workspace version in root Cargo.toml
echo "  Updating Cargo.toml workspace version..."
sed -i.bak 's/^version = ".*"/version = "'"$VERSION"'"/' Cargo.toml
rm -f Cargo.toml.bak

# 2. Update Python pyproject.toml
echo "  Updating core/claim169-python/pyproject.toml..."
sed -i.bak 's/^version = ".*"/version = "'"$VERSION"'"/' core/claim169-python/pyproject.toml
rm -f core/claim169-python/pyproject.toml.bak

# 3. Update TypeScript package.json (using node for reliable JSON editing)
echo "  Updating sdks/typescript/package.json..."
if command -v node &> /dev/null; then
    node -e "
        const fs = require('fs');
        const path = 'sdks/typescript/package.json';
        const pkg = JSON.parse(fs.readFileSync(path, 'utf8'));
        pkg.version = '$VERSION';
        fs.writeFileSync(path, JSON.stringify(pkg, null, 2) + '\n');
    "
else
    # Fallback to sed if node is not available
    sed -i.bak 's/"version": ".*"/"version": "'"$VERSION"'"/' sdks/typescript/package.json
    rm -f sdks/typescript/package.json.bak
fi

# 4. Update Cargo.lock
echo "  Updating Cargo.lock..."
cargo update -w --quiet

# 5. Verify all versions match
echo "Verifying versions..."

CARGO_VERSION=$(grep -m1 '^version = ' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
PYPI_VERSION=$(grep -m1 '^version = ' core/claim169-python/pyproject.toml | sed 's/.*"\(.*\)".*/\1/')

if command -v node &> /dev/null; then
    NPM_VERSION=$(node -p "require('./sdks/typescript/package.json').version")
else
    NPM_VERSION=$(grep '"version"' sdks/typescript/package.json | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
fi

ERRORS=0

if [ "$CARGO_VERSION" != "$VERSION" ]; then
    echo "  ERROR: Cargo.toml version mismatch: $CARGO_VERSION != $VERSION"
    ERRORS=$((ERRORS + 1))
fi

if [ "$PYPI_VERSION" != "$VERSION" ]; then
    echo "  ERROR: pyproject.toml version mismatch: $PYPI_VERSION != $VERSION"
    ERRORS=$((ERRORS + 1))
fi

if [ "$NPM_VERSION" != "$VERSION" ]; then
    echo "  ERROR: package.json version mismatch: $NPM_VERSION != $VERSION"
    ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -gt 0 ]; then
    echo ""
    echo "Version synchronization failed with $ERRORS error(s)"
    exit 1
fi

echo ""
echo "All versions updated to $VERSION"
echo "  Cargo.toml:      $CARGO_VERSION"
echo "  pyproject.toml:  $PYPI_VERSION"
echo "  package.json:    $NPM_VERSION"
