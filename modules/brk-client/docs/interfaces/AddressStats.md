[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddressStats

# Interface: AddressStats

Defined in: [Developer/brk/modules/brk-client/index.js:43](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L43)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:44](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L44)

Bitcoin address string

***

### chainStats

> **chainStats**: [`AddressChainStats`](AddressChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:45](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L45)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats?

> `optional` **mempoolStats**: [`AddressMempoolStats`](AddressMempoolStats.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:46](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L46)

Statistics for unconfirmed transactions in the mempool
