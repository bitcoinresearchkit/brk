[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern3

# Type Alias: SeriesPattern3\<T\>

> **SeriesPattern3**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2184](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L2184)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.minute10

> `readonly` **minute10**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
