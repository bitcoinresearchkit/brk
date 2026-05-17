[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesEndpoint

# Interface: DateSeriesEndpoint\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1676](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1676)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onValue?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1682](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1682)

Fetch all data

#### Parameters

##### onValue?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1683](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1683)

Fetch all data as CSV

#### Returns

`Promise`\<`string`\>

***

### first

> **first**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1679](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1679)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### get

> **get**: (`index`) => [`DateSingleItemBuilder`](DateSingleItemBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1677](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1677)

Get single item at index or Date

#### Parameters

##### index

`number` \| `Date`

#### Returns

[`DateSingleItemBuilder`](DateSingleItemBuilder.md)\<`T`\>

***

### last

> **last**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1680](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1680)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### len

> **len**: () => `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1684](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1684)

Get total number of data points

#### Returns

`Promise`\<`number`\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1687](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1687)

The endpoint path

***

### skip

> **skip**: (`n`) => [`DateSkippedBuilder`](DateSkippedBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1681](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1681)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`DateSkippedBuilder`](DateSkippedBuilder.md)\<`T`\>

***

### slice

> **slice**: (`start?`, `end?`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1678](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1678)

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

Defined in: [Developer/brk/modules/brk-client/index.js:1686](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1686)

Thenable (await endpoint)

***

### version

> **version**: () => `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1685](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1685)

Get the current version of the series

#### Returns

`Promise`\<`number`\>
