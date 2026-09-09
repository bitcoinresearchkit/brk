[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / DateSeriesEndpoint

# Interface: DateSeriesEndpoint\<T\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1741](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1741)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`arg?`, `options?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1747](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1747)

Fetch all data

#### Parameters

##### arg?

[`DateSeriesFetchArg`](../type-aliases/DateSeriesFetchArg.md)\<`T`\>

##### options?

[`ClientFetchOptions`](ClientFetchOptions.md)\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: (`options?`) => `Promise`\<`string`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1748](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1748)

Fetch all data as CSV

#### Parameters

##### options?

[`ClientFetchOptions`](ClientFetchOptions.md)\<`string`\>

#### Returns

`Promise`\<`string`\>

***

### first

> **first**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1744](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1744)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### get

> **get**: (`index`) => [`DateSingleItemBuilder`](DateSingleItemBuilder.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1742](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1742)

Get single item at index or Date

#### Parameters

##### index

`number` \| `Date`

#### Returns

[`DateSingleItemBuilder`](DateSingleItemBuilder.md)\<`T`\>

***

### last

> **last**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1745](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1745)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### len

> **len**: () => `Promise`\<`number`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1749](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1749)

Get total number of data points

#### Returns

`Promise`\<`number`\>

***

### path

> **path**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1752](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1752)

The endpoint path

***

### skip

> **skip**: (`n`) => [`DateSkippedBuilder`](DateSkippedBuilder.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1746](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1746)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`DateSkippedBuilder`](DateSkippedBuilder.md)\<`T`\>

***

### slice

> **slice**: (`start?`, `end?`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1743](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1743)

Slice by index or Date

#### Parameters

##### start?

`number` \| `Date`

##### end?

`number` \| `Date`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1751](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1751)

Thenable (await endpoint)

***

### version

> **version**: () => `Promise`\<`number`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1750](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1750)

Get the current version of the series

#### Returns

`Promise`\<`number`\>
