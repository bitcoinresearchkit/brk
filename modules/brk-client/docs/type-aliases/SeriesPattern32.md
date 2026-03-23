[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern32

# Type Alias: SeriesPattern32\<T\>

> **SeriesPattern32**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1616](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1616)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2wsh\_addr\_index

> `readonly` **p2wsh\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
