[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern23

# Type Alias: SeriesPattern23\<T\>

> **SeriesPattern23**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1889](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1889)

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
