[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / Urpd

# Interface: Urpd

Defined in: [Developer/mono/modules/bitview-client/index.js:1382](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1382)

## Properties

### aggregation

> **aggregation**: [`UrpdAggregation`](../type-aliases/UrpdAggregation.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1386](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1386)

Aggregation strategy applied to the buckets.

***

### buckets

> **buckets**: [`UrpdBucket`](UrpdBucket.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1389](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1389)

***

### close

> **close**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1387](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1387)

Close price on `date`, in USD. Anchor for `unrealized_pnl`.

***

### cohort

> **cohort**: [`Cohort`](../type-aliases/Cohort.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1383](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1383)

***

### date

> **date**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1384](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1384)

***

### totalSupply

> **totalSupply**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1388](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1388)

Sum of `supply` across all buckets, in BTC.

***

### weight

> **weight**: [`UrpdWeight`](../type-aliases/UrpdWeight.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1385](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1385)

Weighting applied to the source supply.
