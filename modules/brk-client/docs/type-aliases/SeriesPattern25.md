[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern25

# Type Alias: SeriesPattern25\<T\>

> **SeriesPattern25**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1775](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1775)

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
