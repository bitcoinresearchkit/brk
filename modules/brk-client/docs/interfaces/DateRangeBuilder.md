[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateRangeBuilder

# Interface: DateRangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1416](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1416)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1417](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1417)

Fetch the range

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1418](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1418)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1419](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1419)

Thenable
