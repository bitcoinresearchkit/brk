docker run \
    --env rpcuser=satoshi \
    --env rpcpassword=nakamoto \
    --env rpcport=localhost \
    --env rpcport=8332 \
    --volume /tmp/kibo/datasets:/container/kibo/datasets \
    --volume /tmp/kibo/price:/container/kibo/price \
    --volume /tmp/kibo/outputs:/container/kibo/parser/out \
    --volume $HOME/Developer/bitcoin:/bitcoin \
    --net=host \
    kibo
