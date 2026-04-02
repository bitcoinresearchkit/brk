[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern16

# Type Alias: SeriesPattern16\<T\>

> **SeriesPattern16**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1747](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1747)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.halving

> `readonly` **halving**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
