[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern12

# Type Alias: SeriesPattern12\<T\>

> **SeriesPattern12**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1856](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1856)

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
