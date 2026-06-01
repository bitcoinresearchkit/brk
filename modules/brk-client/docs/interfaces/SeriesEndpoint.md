[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesEndpoint

# Interface: SeriesEndpoint\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1668](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1668)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onValue?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1674](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1674)

Fetch all data

#### Parameters

##### onValue?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1675](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1675)

Fetch all data as CSV

#### Returns

`Promise`\<`string`\>

***

### first

> **first**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1671](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1671)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### get

> **get**: (`index`) => [`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1669](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1669)

Get single item at index

#### Parameters

##### index

`number`

#### Returns

[`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

***

### last

> **last**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1672](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1672)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### len

> **len**: () => `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1676](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1676)

Get total number of data points

#### Returns

`Promise`\<`number`\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1679](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1679)

The endpoint path

***

### skip

> **skip**: (`n`) => [`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1673](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1673)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

***

### slice

> **slice**: (`start?`, `end?`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1670](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1670)

Slice by index

#### Parameters

##### start?

`number`

##### end?

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1678](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1678)

Thenable (await endpoint)

***

### version

> **version**: () => `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1677](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1677)

Get the current version of the series

#### Returns

`Promise`\<`number`\>
