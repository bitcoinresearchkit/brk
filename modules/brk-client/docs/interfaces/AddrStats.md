[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrStats

# Interface: AddrStats

Defined in: [Developer/brk/modules/brk-client/index.js:43](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L43)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:44](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L44)

Bitcoin address string

***

### chainStats

> **chainStats**: [`AddrChainStats`](AddrChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:45](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L45)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats?

> `optional` **mempoolStats?**: [`AddrMempoolStats`](AddrMempoolStats.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:46](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L46)

Statistics for unconfirmed transactions in the mempool
