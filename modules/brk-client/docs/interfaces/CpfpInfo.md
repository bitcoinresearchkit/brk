[**brk-client**](../README.md)

***

[brk-client](../globals.md) / CpfpInfo

# Interface: CpfpInfo

Defined in: [Developer/brk/modules/brk-client/index.js:355](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L355)

## Properties

### adjustedVsize

> **adjustedVsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:361](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L361)

Adjusted virtual size (accounting for sigops)

***

### ancestors

> **ancestors**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:356](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L356)

Ancestor transactions in the CPFP chain

***

### bestDescendant?

> `optional` **bestDescendant?**: [`CpfpEntry`](CpfpEntry.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:357](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L357)

Best (highest fee rate) descendant, if any

***

### descendants

> **descendants**: [`CpfpEntry`](CpfpEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:358](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L358)

Descendant transactions in the CPFP chain

***

### effectiveFeePerVsize

> **effectiveFeePerVsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:359](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L359)

Effective fee rate considering CPFP relationships (sat/vB)

***

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:360](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L360)

Transaction fee (sats)
