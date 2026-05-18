[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern16

# Type Alias: SeriesPattern16\<T\>

> **SeriesPattern16**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2205](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L2205)

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
