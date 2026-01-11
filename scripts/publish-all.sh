#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

echo "=== Publishing all BRK packages ==="

# Get version from Cargo.toml
VERSION=$(grep 'package.version' "$ROOT_DIR/Cargo.toml" | sed 's/.*= *"//' | sed 's/".*//')
echo "Version: $VERSION"

# Update JS package.json version
sed -i '' 's/"version": "[^"]*"/"version": "'"$VERSION"'"/' "$ROOT_DIR/modules/brk-client/package.json"

# Update Python pyproject.toml version
sed -i '' 's/^version = "[^"]*"/version = "'"$VERSION"'"/' "$ROOT_DIR/packages/brk_client/pyproject.toml"

# 1. Publish Rust crates
echo ""
echo "=== Rust crates ==="
"$SCRIPT_DIR/publish.sh"

# 2. Publish JavaScript package
echo ""
echo "=== JavaScript package ==="
cd "$ROOT_DIR/modules/brk-client"
./scripts/docs.sh
npm publish --access public

# 3. Publish Python package
echo ""
echo "=== Python package ==="
cd "$ROOT_DIR/packages/brk_client"
uvx pydoc-markdown > docs/API.md
uv build
uv publish

echo ""
echo "=== Done! ==="
