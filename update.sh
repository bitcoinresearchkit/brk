#!/usr/bin/env bash

cargo clean
rustup update
cargo upgrade --incompatible
cargo update
