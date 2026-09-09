[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / MempoolInfo

# Interface: MempoolInfo

Defined in: [Developer/mono/modules/bitview-client/index.js:728](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L728)

## Properties

### count

> **count**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:729](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L729)

Number of transactions in the mempool

***

### feeHistogram

> **feeHistogram**: `number`[][]

Defined in: [Developer/mono/modules/bitview-client/index.js:732](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L732)

Fee histogram: `[[fee_rate, vsize], ...]` sorted by descending fee rate

***

### totalFee

> **totalFee**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:731](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L731)

Total fees of all transactions in the mempool (satoshis)

***

### vsize

> **vsize**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:730](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L730)

Total virtual size of all transactions in the mempool (vbytes)
