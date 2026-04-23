[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Urpd

# Interface: Urpd

Defined in: [Developer/brk/modules/brk-client/index.js:1157](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1157)

## Properties

### aggregation

> **aggregation**: [`UrpdAggregation`](../type-aliases/UrpdAggregation.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1160](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1160)

Aggregation strategy applied to the buckets.

***

### buckets

> **buckets**: [`UrpdBucket`](UrpdBucket.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1163](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1163)

***

### close

> **close**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1161](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1161)

Close price on `date`, in USD. Anchor for `unrealized_pnl`.

***

### cohort

> **cohort**: [`Cohort`](../type-aliases/Cohort.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1158](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1158)

***

### date

> **date**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1159](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1159)

***

### totalSupply

> **totalSupply**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1162](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1162)

Sum of `supply` across all buckets, in BTC.
