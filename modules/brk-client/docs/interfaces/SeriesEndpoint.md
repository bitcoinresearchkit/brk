[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesEndpoint

# Interface: SeriesEndpoint\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1417](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1417)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1423](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1423)

Fetch all data

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1424](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1424)

Fetch all data as CSV

#### Returns

`Promise`\<`string`\>

***

### first

> **first**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1420](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1420)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### get

> **get**: (`index`) => [`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1418](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1418)

Get single item at index

#### Parameters

##### index

`number`

#### Returns

[`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

***

### last

> **last**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1421](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1421)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1426](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1426)

The endpoint path

***

### skip

> **skip**: (`n`) => [`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1422](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1422)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

***

### slice

> **slice**: (`start?`, `end?`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1419](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1419)

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

Defined in: [Developer/brk/modules/brk-client/index.js:1425](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1425)

Thenable (await endpoint)
