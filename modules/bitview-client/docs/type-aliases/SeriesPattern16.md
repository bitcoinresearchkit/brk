[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / SeriesPattern16

# Type Alias: SeriesPattern16\<T\>

> **SeriesPattern16**\<`T`\> = `object`

Defined in: [Developer/mono/modules/bitview-client/index.js:2469](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L2469)

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
