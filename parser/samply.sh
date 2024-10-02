#!/usr/bin/env bash

echo "Increasing limit of opened files..."
ulimit -n 1000000

# Needed because the datasets tree is too big lol
echo "Increasing stack size..."
ulimit -s $(ulimit -Hs)

cargo build --profile profiling && samply record ./target/profiling/parser "$HOME/Developer/bitcoin"
