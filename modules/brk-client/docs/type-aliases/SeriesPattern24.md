[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern24

# Type Alias: SeriesPattern24\<T\>

> **SeriesPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1781](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L1781)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2a\_addr\_index

> `readonly` **p2a\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
