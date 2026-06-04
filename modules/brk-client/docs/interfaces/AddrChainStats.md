[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrChainStats

# Interface: AddrChainStats

Defined in: [Developer/brk/modules/brk-client/index.js:23](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L23)

## Properties

### fundedTxoCount

> **fundedTxoCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:24](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L24)

Total number of transaction outputs that funded this address

***

### fundedTxoSum

> **fundedTxoSum**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:25](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L25)

Total amount in satoshis received by this address across all funded outputs

***

### realizedPrice

> **realizedPrice**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:30](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L30)

Realized price (average cost basis) in USD

***

### spentTxoCount

> **spentTxoCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:26](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L26)

Total number of transaction outputs spent from this address

***

### spentTxoSum

> **spentTxoSum**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:27](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L27)

Total amount in satoshis spent from this address

***

### txCount

> **txCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:28](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L28)

Total number of confirmed transactions involving this address

***

### typeIndex

> **typeIndex**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:29](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L29)

Index of this address within its type on the blockchain
