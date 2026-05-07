[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern10

# Type Alias: SeriesPattern10\<T\>

> **SeriesPattern10**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2128](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L2128)

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
