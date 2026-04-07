[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern7

# Type Alias: SeriesPattern7\<T\>

> **SeriesPattern7**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1785](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L1785)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.hour12

> `readonly` **hour12**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

### get

> **get**: (`index`) => [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\> \| `undefined`

#### Parameters

##### index

[`Index`](Index.md)

#### Returns

[`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\> \| `undefined`

### indexes

> **indexes**: () => readonly [`Index`](Index.md)[]

#### Returns

readonly [`Index`](Index.md)[]

### name

> **name**: `string`
