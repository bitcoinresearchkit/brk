[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern23

# Type Alias: SeriesPattern23\<T\>

> **SeriesPattern23**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1589](https://github.com/bitcoinresearchkit/brk/blob/19d4a193ffb7d8bbcaf9d0ccaccf8e90edff9e28/modules/brk-client/index.js#L1589)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.op\_return\_index

> `readonly` **op\_return\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
