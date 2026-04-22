[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1448](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1448)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1450](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1450)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1449](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1449)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1451](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1451)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
