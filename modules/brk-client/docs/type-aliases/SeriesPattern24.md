[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern24

# Type Alias: SeriesPattern24\<T\>

> **SeriesPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1772](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1772)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2a\_addr\_index

> `readonly` **p2a\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
