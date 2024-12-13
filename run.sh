#!/usr/bin/env bash

# https://stackoverflow.com/questions/31389483/find-and-delete-file-or-folder-older-than-x-days

if command -v ulimit &> /dev/null; then
    echo "Increasing limit of opened files..."
    # ulimit -n 1000000 # Can't be $(ulimit -Hn), bitcoind needs some too !

    # Needed because the datasets tree is too big lol
    echo "Increasing stack size..."
    ulimit -s $(ulimit -Hs)
fi

# For Mac OS users
if [ "$(uname)" == "Darwin" ]; then
    if mdutil -s / | grep "enabled"; then
        echo "Disabling spotlight indexing..."
        sudo mdutil -a -i off &> /dev/null
    fi

    echo "Thinning local TimeMachine snapshots..."
    tmutil thinlocalsnapshots / &> /dev/null
fi

cargo run -r -- "$@"
