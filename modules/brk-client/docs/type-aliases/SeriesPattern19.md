[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern19

# Type Alias: SeriesPattern19\<T\>

> **SeriesPattern19**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1877](https://github.com/bitcoinresearchkit/brk/blob/e4496742a4964a986078f3a65d3bfdc1e47a5eba/modules/brk-client/index.js#L1877)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.tx\_index

> `readonly` **tx\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
