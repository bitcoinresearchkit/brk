[**brk-client**](../README.md)

***

[brk-client](../globals.md) / CpfpInfo

# Interface: CpfpInfo

Defined in: [Developer/brk/modules/brk-client/index.js:374](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L374)

## Properties

### adjustedVsize?

> `optional` **adjustedVsize?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:380](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L380)

Adjusted virtual size (accounting for sigops)

***

### ancestors

> **ancestors**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:375](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L375)

Ancestor transactions in the CPFP chain

***

### bestDescendant?

> `optional` **bestDescendant?**: [`CpfpEntry`](CpfpEntry.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:376](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L376)

Best (highest fee rate) descendant, if any

***

### descendants

> **descendants**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:377](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L377)

Descendant transactions in the CPFP chain

***

### effectiveFeePerVsize?

> `optional` **effectiveFeePerVsize?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:378](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L378)

Effective fee rate considering CPFP relationships (sat/vB)

***

### fee?

> `optional` **fee?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:379](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L379)

Transaction fee (sats)
