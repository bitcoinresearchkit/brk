[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1646](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1646)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1648](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1648)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1647](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1647)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1649](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1649)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
