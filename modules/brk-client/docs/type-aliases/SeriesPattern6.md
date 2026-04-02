[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern6

# Type Alias: SeriesPattern6\<T\>

> **SeriesPattern6**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1717](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1717)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.hour4

> `readonly` **hour4**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
