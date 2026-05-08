[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern4

# Type Alias: SeriesPattern4\<T\>

> **SeriesPattern4**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2107](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L2107)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.minute30

> `readonly` **minute30**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
