[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolInfo

# Interface: MempoolInfo

Defined in: [Developer/brk/modules/brk-client/index.js:625](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L625)

## Properties

### count

> **count**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:626](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L626)

Number of transactions in the mempool

***

### feeHistogram

> **feeHistogram**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:629](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L629)

Fee histogram: `[[fee_rate, vsize], ...]` sorted by descending fee rate

#### Index Signature

\[`key`: `string`\]: `number`

***

### totalFee

> **totalFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:628](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L628)

Total fees of all transactions in the mempool (satoshis)

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:627](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L627)

Total virtual size of all transactions in the mempool (vbytes)
