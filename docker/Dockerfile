FROM rust:1.81

ENV rpcconnect=localhost
ENV rpcport=8332
ENV rpcuser=satoshi
ENV rpcpassword=nakamoto

WORKDIR /
COPY . .

CMD exec cargo run -r --manifest-path /kibo/parser/Cargo.toml -- --datadir=/bitcoin --rpcconnect=${rpcconnect} --rpcport=${rpcport} --rpcuser=${rpcuser} --rpcpassword=${rpcpassword}
