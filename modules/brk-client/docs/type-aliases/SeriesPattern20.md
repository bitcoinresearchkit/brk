[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern20

# Type Alias: SeriesPattern20\<T\>

> **SeriesPattern20**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1929](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L1929)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.txin\_index

> `readonly` **txin\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
