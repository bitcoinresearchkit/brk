#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

echo "=== BRK Release ==="
echo ""

# Check if version argument provided
if [ -z "$1" ]; then
    echo "Usage: $0 <version|bump>"
    echo "Examples:"
    echo "  $0 0.1.0-alpha.3"
    echo "  $0 patch"
    echo "  $0 minor"
    echo "  $0 major"
    exit 1
fi

RELEASE_ARG="$1"
echo "Release argument: $RELEASE_ARG"
echo ""

# Load tokens
if [ -f "$SCRIPT_DIR/.tokens" ]; then
    source "$SCRIPT_DIR/.tokens"
fi

# ============================================================================
# 0. VERIFY TOKENS
# ============================================================================

echo "=== Verifying tokens ==="
echo ""

echo "--- npm ---"
npm whoami || { echo "npm not authenticated. Run: npm login"; exit 1; }
echo ""

echo "--- PyPI ---"
if [ -z "$UV_PUBLISH_TOKEN" ]; then
    echo "UV_PUBLISH_TOKEN not set. Add it to scripts/.tokens"
    exit 1
fi
echo "OK"
echo ""

# ============================================================================
# 1. TESTS
# ============================================================================

echo "=== Running tests ==="
echo ""

echo "--- Rust ---"
cd "$ROOT_DIR"
cargo test --workspace --exclude brk_playground
echo ""

echo "--- JavaScript ---"
cd "$ROOT_DIR/modules/brk-client"
npm test
echo ""

echo "--- Python ---"
cd "$ROOT_DIR/packages/brk_client"
uv run pytest tests/ -s
echo ""

# ============================================================================
# 2. BUILD
# ============================================================================

echo "=== Building ==="
echo ""

echo "--- Rust ---"
cd "$ROOT_DIR"
cargo build --workspace --release
echo ""

echo "--- JavaScript ---"
cd "$ROOT_DIR/modules/brk-client"
# JS doesn't need build step, just verify it loads
node -e "import('./index.js')"
echo "OK"
echo ""

echo "--- Python ---"
cd "$ROOT_DIR/packages/brk_client"
uv build
echo ""

# ============================================================================
# 3. GENERATE DOCS
# ============================================================================

echo "=== Generating docs ==="
echo ""

echo "--- JavaScript ---"
"$SCRIPT_DIR/js-docs.sh"
echo ""

echo "--- Python ---"
"$SCRIPT_DIR/python-docs.sh"
echo ""

# Commit generated docs
cd "$ROOT_DIR"
git add -A
git commit -m "docs: update generated docs" || echo "No doc changes to commit"
echo ""

# ============================================================================
# 4. CARGO RELEASE (Rust crates)
# ============================================================================

echo "=== Rust release ==="
echo ""

cd "$ROOT_DIR"

# Verify all crates package correctly
# Note: --no-verify skips rebuild check due to version collision with crates.io
# The cargo build --workspace --release step above already verified compilation
cargo package --workspace --allow-dirty --exclude brk_playground --no-verify

# Version bump, commit, and tag (but don't publish yet)
cargo release "$RELEASE_ARG" --execute --no-publish --no-confirm

# Publish crates with retry logic for rate limits
"$SCRIPT_DIR/rust-publish.sh"

# Extract actual version from Cargo.toml after release
VERSION=$(grep '^package.version' "$ROOT_DIR/Cargo.toml" | sed 's/.*= *"//' | sed 's/".*//')
echo ""
echo "Released Rust crates at version: $VERSION"

# ============================================================================
# 5. JAVASCRIPT PACKAGE
# ============================================================================

echo ""
echo "=== JavaScript package ==="
"$SCRIPT_DIR/js-publish.sh" "$VERSION"
echo ""

# ============================================================================
# 6. PYTHON PACKAGE
# ============================================================================

echo ""
echo "=== Python package ==="
"$SCRIPT_DIR/python-publish.sh" "$VERSION"
echo ""

# ============================================================================
# DONE
# ============================================================================

echo "=== Done! ==="
echo "Released v$VERSION"
