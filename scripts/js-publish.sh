#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

VERSION="$1"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

cd "$ROOT_DIR/modules/brk-client"

# Update version in package.json
sed -i '' 's/"version": "[^"]*"/"version": "'"$VERSION"'"/' package.json
echo "Updated package.json to $VERSION"

npm publish --access public
