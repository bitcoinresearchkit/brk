[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern16

# Type Alias: SeriesPattern16\<T\>

> **SeriesPattern16**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1568](https://github.com/bitcoinresearchkit/brk/blob/19d4a193ffb7d8bbcaf9d0ccaccf8e90edff9e28/modules/brk-client/index.js#L1568)

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
