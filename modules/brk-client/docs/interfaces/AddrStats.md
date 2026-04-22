[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrStats

# Interface: AddrStats

Defined in: [Developer/brk/modules/brk-client/index.js:46](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L46)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:47](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L47)

Bitcoin address string

***

### addrType

> **addrType**: [`OutputType`](../type-aliases/OutputType.md)

Defined in: [Developer/brk/modules/brk-client/index.js:48](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L48)

Address type (p2pkh, p2sh, v0_p2wpkh, v0_p2wsh, v1_p2tr, etc.)

***

### chainStats

> **chainStats**: [`AddrChainStats`](AddrChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:49](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L49)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats?

> `optional` **mempoolStats?**: [`AddrMempoolStats`](AddrMempoolStats.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:50](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L50)

Statistics for unconfirmed transactions in the mempool
