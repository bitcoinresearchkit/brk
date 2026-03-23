[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1163](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1163)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1165](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1165)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1164](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1164)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1166](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1166)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
