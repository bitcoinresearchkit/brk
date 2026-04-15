[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1410](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1410)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1412](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1412)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1411](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1411)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1413](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1413)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
