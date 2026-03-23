[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern24

# Type Alias: SeriesPattern24\<T\>

> **SeriesPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1592](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1592)

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
