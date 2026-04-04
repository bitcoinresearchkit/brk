[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern9

# Type Alias: SeriesPattern9\<T\>

> **SeriesPattern9**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1736](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L1736)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.day3

> `readonly` **day3**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
