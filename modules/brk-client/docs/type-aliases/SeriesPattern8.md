[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern8

# Type Alias: SeriesPattern8\<T\>

> **SeriesPattern8**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1844](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1844)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.day1

> `readonly` **day1**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
