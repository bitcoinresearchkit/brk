FROM rust:1.81

ENV rpcconnect=localhost
ENV rpcport=8332
ENV rpcuser=satoshi
ENV rpcpassword=nakamoto

WORKDIR /container
COPY . .

CMD ["sh", "-c", "bash cmd.sh ${rpcconnect} ${rpcport} ${rpcuser} ${rpcpassword}"]
