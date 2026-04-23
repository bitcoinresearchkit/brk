[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern14

# Type Alias: SeriesPattern14\<T\>

> **SeriesPattern14**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1862](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1862)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.year1

> `readonly` **year1**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
