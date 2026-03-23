[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateRangeBuilder

# Interface: DateRangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1237](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1237)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1238](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1238)

Fetch the range

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1239](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1239)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1240](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1240)

Thenable
