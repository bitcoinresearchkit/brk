#!/usr/bin/env bash

cargo check

cd crates/brk

cd ../brk_core
cargo publish --allow-dirty

cd ../brk_exit
cargo publish --allow-dirty

cd ../brk_vec
cargo publish --allow-dirty

cd ../brk_logger
cargo publish --allow-dirty

cd ../brk_indexer
cargo publish --allow-dirty

cd ../brk_parser
cargo publish --allow-dirty

cd ../brk_fetcher
cargo publish --allow-dirty

cd ../brk_computer
cargo publish --allow-dirty

cd ../brk_query
cargo publish --allow-dirty

cd ../brk_server
cargo publish --allow-dirty

cd ../brk_cli
cargo publish --allow-dirty

cd ../brk
cargo publish --allow-dirty

cd ../..
