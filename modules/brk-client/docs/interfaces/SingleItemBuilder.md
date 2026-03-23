[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SingleItemBuilder

# Interface: SingleItemBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1205](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1205)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1206](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1206)

Fetch the item

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1207](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1207)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1208](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1208)

Thenable
