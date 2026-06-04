[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Urpd

# Interface: Urpd

Defined in: [Developer/brk/modules/brk-client/index.js:1349](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1349)

## Properties

### aggregation

> **aggregation**: [`UrpdAggregation`](../type-aliases/UrpdAggregation.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1352](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1352)

Aggregation strategy applied to the buckets.

***

### buckets

> **buckets**: [`UrpdBucket`](UrpdBucket.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1355](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1355)

***

### close

> **close**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1353](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1353)

Close price on `date`, in USD. Anchor for `unrealized_pnl`.

***

### cohort

> **cohort**: [`Cohort`](../type-aliases/Cohort.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1350](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1350)

***

### date

> **date**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1351](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1351)

***

### totalSupply

> **totalSupply**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1354](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1354)

Sum of `supply` across all buckets, in BTC.
