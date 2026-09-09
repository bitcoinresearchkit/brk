[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / DetailedSeriesCount

# Interface: DetailedSeriesCount

Defined in: [Developer/mono/modules/bitview-client/index.js:505](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L505)

## Properties

### byDb

> **byDb**: `object`

Defined in: [Developer/mono/modules/bitview-client/index.js:510](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L510)

Per-database breakdown of counts.

#### Index Signature

\[`key`: `string`\]: [`SeriesCount`](SeriesCount.md)

***

### distinct

> **distinct**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:506](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L506)

Number of unique series available (e.g., realized_price, market_cap)

***

### lazy

> **lazy**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:508](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L508)

Number of lazy (computed on-the-fly) series-index combinations

***

### stored

> **stored**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:509](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L509)

Number of eager (stored on disk) series-index combinations

***

### total

> **total**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:507](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L507)

Total number of series-index combinations across all timeframes
