[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern24

# Type Alias: SeriesPattern24\<T\>

> **SeriesPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1771](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1771)

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
