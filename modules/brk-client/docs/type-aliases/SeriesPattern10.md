[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern10

# Type Alias: SeriesPattern10\<T\>

> **SeriesPattern10**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1730](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1730)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.week1

> `readonly` **week1**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
