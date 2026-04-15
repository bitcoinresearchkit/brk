[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern13

# Type Alias: SeriesPattern13\<T\>

> **SeriesPattern13**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1821](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1821)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.month6

> `readonly` **month6**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
