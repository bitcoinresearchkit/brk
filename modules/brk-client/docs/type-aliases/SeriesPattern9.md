[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern9

# Type Alias: SeriesPattern9\<T\>

> **SeriesPattern9**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1753](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1753)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.day3

> `readonly` **day3**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
