#!/usr/bin/env bash

cd kibo/parser

./run.sh \
  --datadir=/bitcoin \
  --rpcconnect=$1 \
  --rpcport=$2 \
  --rpcuser=$3 \
  --rpcpassword=$4

# cd ../server
# ./run.sh &
