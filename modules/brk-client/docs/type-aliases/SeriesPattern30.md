[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern30

# Type Alias: SeriesPattern30\<T\>

> **SeriesPattern30**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1790](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1790)

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
