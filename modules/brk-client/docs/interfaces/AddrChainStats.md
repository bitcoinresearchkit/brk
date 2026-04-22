[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrChainStats

# Interface: AddrChainStats

Defined in: [Developer/brk/modules/brk-client/index.js:16](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L16)

## Properties

### fundedTxoCount

> **fundedTxoCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:17](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L17)

Total number of transaction outputs that funded this address

***

### fundedTxoSum

> **fundedTxoSum**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:18](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L18)

Total amount in satoshis received by this address across all funded outputs

***

### realizedPrice

> **realizedPrice**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:23](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L23)

Realized price (average cost basis) in USD

***

### spentTxoCount

> **spentTxoCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:19](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L19)

Total number of transaction outputs spent from this address

***

### spentTxoSum

> **spentTxoSum**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:20](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L20)

Total amount in satoshis spent from this address

***

### txCount

> **txCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:21](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L21)

Total number of confirmed transactions involving this address

***

### typeIndex

> **typeIndex**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:22](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L22)

Index of this address within its type on the blockchain
