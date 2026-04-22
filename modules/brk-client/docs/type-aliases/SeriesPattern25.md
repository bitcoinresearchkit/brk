[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern25

# Type Alias: SeriesPattern25\<T\>

> **SeriesPattern25**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1895](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1895)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2ms\_output\_index

> `readonly` **p2ms\_output\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
