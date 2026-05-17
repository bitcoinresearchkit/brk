[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern12

# Type Alias: SeriesPattern12\<T\>

> **SeriesPattern12**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2193](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L2193)

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
