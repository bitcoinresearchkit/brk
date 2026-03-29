[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrStats

# Interface: AddrStats

Defined in: [Developer/brk/modules/brk-client/index.js:43](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L43)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:44](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L44)

Bitcoin address string

***

### chainStats

> **chainStats**: [`AddrChainStats`](AddrChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:45](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L45)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats?

> `optional` **mempoolStats?**: [`AddrMempoolStats`](AddrMempoolStats.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:46](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L46)

Statistics for unconfirmed transactions in the mempool
