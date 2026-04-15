[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolInfo

# Interface: MempoolInfo

Defined in: [Developer/brk/modules/brk-client/index.js:639](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L639)

## Properties

### count

> **count**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:640](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L640)

Number of transactions in the mempool

***

### feeHistogram

> **feeHistogram**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:643](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L643)

Fee histogram: `[[fee_rate, vsize], ...]` sorted by descending fee rate

#### Index Signature

\[`key`: `string`\]: `number`

***

### totalFee

> **totalFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:642](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L642)

Total fees of all transactions in the mempool (satoshis)

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:641](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L641)

Total virtual size of all transactions in the mempool (vbytes)
