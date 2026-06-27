[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolInfo

# Interface: MempoolInfo

Defined in: [Developer/brk/modules/brk-client/index.js:754](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L754)

## Properties

### count

> **count**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:755](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L755)

Number of transactions in the mempool

***

### feeHistogram

> **feeHistogram**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:758](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L758)

Fee histogram: `[[fee_rate, vsize], ...]` sorted by descending fee rate

#### Index Signature

\[`key`: `string`\]: `number`

***

### totalFee

> **totalFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:757](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L757)

Total fees of all transactions in the mempool (satoshis)

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:756](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L756)

Total virtual size of all transactions in the mempool (vbytes)
