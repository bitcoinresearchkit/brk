[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1327](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1327)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1329](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1329)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1328](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1328)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1330](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1330)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
