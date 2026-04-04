[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern30

# Type Alias: SeriesPattern30\<T\>

> **SeriesPattern30**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1792](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1792)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2tr\_addr\_index

> `readonly` **p2tr\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
