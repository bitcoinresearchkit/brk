[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrStats

# Interface: AddrStats

Defined in: [Developer/brk/modules/brk-client/index.js:46](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L46)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:47](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L47)

Bitcoin address string

***

### addrType

> **addrType**: [`OutputType`](../type-aliases/OutputType.md)

Defined in: [Developer/brk/modules/brk-client/index.js:48](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L48)

Address type (p2pkh, p2sh, v0_p2wpkh, v0_p2wsh, v1_p2tr, etc.)

***

### chainStats

> **chainStats**: [`AddrChainStats`](AddrChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:49](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L49)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats?

> `optional` **mempoolStats?**: [`AddrMempoolStats`](AddrMempoolStats.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:50](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L50)

Statistics for unconfirmed transactions in the mempool
