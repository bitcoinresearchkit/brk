[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrStats

# Interface: AddrStats

Defined in: [Developer/brk/modules/brk-client/index.js:53](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L53)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:54](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L54)

Bitcoin address string

***

### addrType

> **addrType**: [`OutputType`](../type-aliases/OutputType.md)

Defined in: [Developer/brk/modules/brk-client/index.js:55](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L55)

Address type (p2pkh, p2sh, v0_p2wpkh, v0_p2wsh, v1_p2tr, etc.)

***

### chainStats

> **chainStats**: [`AddrChainStats`](AddrChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:56](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L56)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats

> **mempoolStats**: [`AddrMempoolStats`](AddrMempoolStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:57](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L57)

Statistics for unconfirmed transactions in the mempool
