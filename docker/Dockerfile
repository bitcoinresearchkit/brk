FROM rust:1.81

ENV rpcuser=satoshi
ENV rpcpassword=nakamoto
ENV rpcport=8332

WORKDIR /container
COPY . .

CMD ["sh", "-c", "bash cmd.sh ${rpcuser} ${rpcpassword} ${rpcport}"]
