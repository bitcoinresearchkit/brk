[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1345](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1345)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1347](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1347)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1346](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1346)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1348](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1348)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
