[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern23

# Type Alias: SeriesPattern23\<T\>

> **SeriesPattern23**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2244](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L2244)

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
