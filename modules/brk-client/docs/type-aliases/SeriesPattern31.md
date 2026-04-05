[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern31

# Type Alias: SeriesPattern31\<T\>

> **SeriesPattern31**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1819](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1819)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2wpkh\_addr\_index

> `readonly` **p2wpkh\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
