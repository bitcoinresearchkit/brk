[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern8

# Type Alias: SeriesPattern8\<T\>

> **SeriesPattern8**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2122](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L2122)

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
