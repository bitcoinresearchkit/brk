[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern15

# Type Alias: SeriesPattern15\<T\>

> **SeriesPattern15**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1809](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1809)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.year10

> `readonly` **year10**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
