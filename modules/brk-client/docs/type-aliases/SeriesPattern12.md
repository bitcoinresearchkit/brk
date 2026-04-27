[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern12

# Type Alias: SeriesPattern12\<T\>

> **SeriesPattern12**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1905](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1905)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.month3

> `readonly` **month3**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
