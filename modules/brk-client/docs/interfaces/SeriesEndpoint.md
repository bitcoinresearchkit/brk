[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesEndpoint

# Interface: SeriesEndpoint\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1359](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1359)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1365](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1365)

Fetch all data

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1366](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1366)

Fetch all data as CSV

#### Returns

`Promise`\<`string`\>

***

### first

> **first**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1362](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1362)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### get

> **get**: (`index`) => [`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1360](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1360)

Get single item at index

#### Parameters

##### index

`number`

#### Returns

[`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

***

### last

> **last**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1363](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1363)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1368](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1368)

The endpoint path

***

### skip

> **skip**: (`n`) => [`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1364](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1364)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

***

### slice

> **slice**: (`start?`, `end?`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1361](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1361)

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

Defined in: [Developer/brk/modules/brk-client/index.js:1367](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1367)

Thenable (await endpoint)
