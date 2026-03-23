[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern28

# Type Alias: SeriesPattern28\<T\>

> **SeriesPattern28**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1604](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1604)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2pkh\_addr\_index

> `readonly` **p2pkh\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
