docker run \
    --env rpcuser=satoshi \
    --env rpcpassword=nakamoto \
    --env rpcport=localhost \
    --env rpcport=8332 \
    --volume /tmp/kibo/datasets:/kibo/datasets \
    --volume /tmp/kibo/price:/kibo/price \
    --volume /tmp/kibo/outputs:/kibo/parser/out \
    --volume $HOME/Developer/bitcoin:/bitcoin \
    --net=host \
    kibo-parser
