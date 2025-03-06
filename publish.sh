#!/usr/bin/env bash

cargo build --all-targets

cd crates/brk

cd ../brk_core
cargo publish

cd ../brk_exit
cargo publish

cd ../brk_vec
cargo publish

cd ../brk_logger
cargo publish

cd ../brk_indexer
cargo publish

cd ../brk_parser
cargo publish

cd ../brk_fetcher
cargo publish

cd ../brk_computer
cargo publish

cd ../brk_query
cargo publish

cd ../brk_server
cargo publish

cd ../brk_cli
cargo publish

cd ../brk
cargo publish

cd ../..
