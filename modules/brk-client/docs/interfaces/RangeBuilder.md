[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RangeBuilder

# Interface: RangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1471](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1471)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1472](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1472)

Fetch the range

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1473](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1473)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1474](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1474)

Thenable
