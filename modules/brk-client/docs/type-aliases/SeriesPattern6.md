[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern6

# Type Alias: SeriesPattern6\<T\>

> **SeriesPattern6**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1718](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1718)

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
