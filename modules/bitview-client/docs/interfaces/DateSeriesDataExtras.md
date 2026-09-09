[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / DateSeriesDataExtras

# Interface: DateSeriesDataExtras\<T\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1711](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1711)

## Type Parameters

### T

`T`

## Properties

### dateEntries

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/mono/modules/bitview-client/index.js:1713](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1713)

Get [date, value] pairs

#### Returns

\[`Date`, `T`\][]

***

### dates

> **dates**: () => `Date`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1712](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1712)

Get dates for each data point

#### Returns

`Date`[]

***

### toDateMap

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1714](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1714)

Convert to Map<date, value>

#### Returns

`Map`\<`Date`, `T`\>
