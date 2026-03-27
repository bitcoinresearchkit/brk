[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern5

# Type Alias: SeriesPattern5\<T\>

> **SeriesPattern5**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1535](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L1535)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.hour1

> `readonly` **hour1**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
