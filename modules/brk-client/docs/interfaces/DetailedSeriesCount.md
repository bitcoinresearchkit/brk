[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DetailedSeriesCount

# Interface: DetailedSeriesCount

Defined in: [Developer/brk/modules/brk-client/index.js:385](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L385)

## Properties

### byDb

> **byDb**: `object`

Defined in: [Developer/brk/modules/brk-client/index.js:390](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L390)

Per-database breakdown of counts

#### Index Signature

\[`key`: `string`\]: [`SeriesCount`](SeriesCount.md)

***

### distinctSeries

> **distinctSeries**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:386](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L386)

Number of unique series available (e.g., realized_price, market_cap)

***

### lazyEndpoints

> **lazyEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:388](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L388)

Number of lazy (computed on-the-fly) series-index combinations

***

### storedEndpoints

> **storedEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:389](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L389)

Number of eager (stored on disk) series-index combinations

***

### totalEndpoints

> **totalEndpoints**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:387](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L387)

Total number of series-index combinations across all timeframes
