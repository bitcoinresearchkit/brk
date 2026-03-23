[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern8

# Type Alias: SeriesPattern8\<T\>

> **SeriesPattern8**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1544](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1544)

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
