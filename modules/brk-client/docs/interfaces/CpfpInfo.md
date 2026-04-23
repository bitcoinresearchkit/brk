[**brk-client**](../README.md)

***

[brk-client](../globals.md) / CpfpInfo

# Interface: CpfpInfo

Defined in: [Developer/brk/modules/brk-client/index.js:373](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L373)

## Properties

### adjustedVsize?

> `optional` **adjustedVsize?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:379](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L379)

Adjusted virtual size (accounting for sigops)

***

### ancestors

> **ancestors**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:374](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L374)

Ancestor transactions in the CPFP chain

***

### bestDescendant?

> `optional` **bestDescendant?**: [`CpfpEntry`](CpfpEntry.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:375](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L375)

Best (highest fee rate) descendant, if any

***

### descendants

> **descendants**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:376](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L376)

Descendant transactions in the CPFP chain

***

### effectiveFeePerVsize?

> `optional` **effectiveFeePerVsize?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:377](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L377)

Effective fee rate considering CPFP relationships (sat/vB)

***

### fee?

> `optional` **fee?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:378](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L378)

Transaction fee (sats)
