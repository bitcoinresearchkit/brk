[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern15

# Type Alias: SeriesPattern15\<T\>

> **SeriesPattern15**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1827](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1827)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.year10

> `readonly` **year10**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
