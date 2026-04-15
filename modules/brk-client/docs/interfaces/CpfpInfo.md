[**brk-client**](../README.md)

***

[brk-client](../globals.md) / CpfpInfo

# Interface: CpfpInfo

Defined in: [Developer/brk/modules/brk-client/index.js:387](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L387)

## Properties

### adjustedVsize?

> `optional` **adjustedVsize?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:393](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L393)

Adjusted virtual size (accounting for sigops)

***

### ancestors

> **ancestors**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:388](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L388)

Ancestor transactions in the CPFP chain

***

### bestDescendant?

> `optional` **bestDescendant?**: [`CpfpEntry`](CpfpEntry.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:389](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L389)

Best (highest fee rate) descendant, if any

***

### descendants

> **descendants**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:390](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L390)

Descendant transactions in the CPFP chain

***

### effectiveFeePerVsize?

> `optional` **effectiveFeePerVsize?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:391](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L391)

Effective fee rate considering CPFP relationships (sat/vB)

***

### fee?

> `optional` **fee?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:392](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L392)

Transaction fee (sats)
