#!/usr/bin/env bash

if cargo watch --help &> /dev/null; then
    TRIGGER="./in/datasets_len.txt"

    if [ ! -f "$TRIGGER" ]; then
        mkdir "./in"
        echo "0" > $TRIGGER
    fi

    cargo watch --no-vcs-ignores -w "./src" -w "$TRIGGER" -x "run -r"
else
    cargo run -r
fi
