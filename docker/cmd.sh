#!/usr/bin/env bash

cd kibo/parser
./run.sh \
  --datadir=/bitcoin \
  --rpcuser=$1 \
  --rpcpassword=$2 \
  --rpcport=$3

# cd ../server
# ./run.sh &
