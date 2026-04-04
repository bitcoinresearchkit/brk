[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RangeBuilder

# Interface: RangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1413](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1413)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1414](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1414)

Fetch the range

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1415](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1415)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1416](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1416)

Thenable
