[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern15

# Type Alias: SeriesPattern15\<T\>

> **SeriesPattern15**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1865](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1865)

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
