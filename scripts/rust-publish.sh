#!/bin/bash
set -e

# Order determined by topological sort of dependency graph
CRATES=(
    brk_alloc
    brk_error
    brk_logger
    brk_types
    brk_fetcher
    brk_rpc
    brk_mempool
    brk_reader
    brk_iterator
    brk_store
    brk_traversable_derive
    brk_traversable
    brk_bencher
    brk_cohort
    brk_indexer
    brk_oracle
    brk_computer
    brk_query
    brk_bindgen
    brk_website
    brk_server
    brk_client
    brk
    brk_cli
)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATES_DIR="$SCRIPT_DIR/../crates"

cd "$CRATES_DIR" || { echo "Failed to cd to crates directory"; exit 1; }
echo "Working from: $(pwd)"

for crate in "${CRATES[@]}"; do
    cd "$crate"
    cargo publish --allow-dirty --color=always 2>&1 | tee /tmp/publish_$$.log
    if [ ${PIPESTATUS[0]} -ne 0 ]; then
        if ! grep -q "already exists on" /tmp/publish_$$.log; then
            rm -f /tmp/publish_$$.log
            exit 1
        fi
    fi
    rm -f /tmp/publish_$$.log
    cd ..
    echo ""
done

echo "Done!"
