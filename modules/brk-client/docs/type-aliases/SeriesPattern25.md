[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern25

# Type Alias: SeriesPattern25\<T\>

> **SeriesPattern25**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1839](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1839)

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
