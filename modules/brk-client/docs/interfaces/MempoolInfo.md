[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolInfo

# Interface: MempoolInfo

Defined in: [Developer/brk/modules/brk-client/index.js:638](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L638)

## Properties

### count

> **count**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:639](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L639)

Number of transactions in the mempool

***

### feeHistogram

> **feeHistogram**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:642](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L642)

Fee histogram: `[[fee_rate, vsize], ...]` sorted by descending fee rate

#### Index Signature

\[`key`: `string`\]: `number`

***

### totalFee

> **totalFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:641](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L641)

Total fees of all transactions in the mempool (satoshis)

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:640](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L640)

Total virtual size of all transactions in the mempool (vbytes)
