[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern17

# Type Alias: SeriesPattern17\<T\>

> **SeriesPattern17**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2226](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L2226)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.epoch

> `readonly` **epoch**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
