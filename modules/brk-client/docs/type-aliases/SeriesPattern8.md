[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern8

# Type Alias: SeriesPattern8\<T\>

> **SeriesPattern8**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1893](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1893)

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
