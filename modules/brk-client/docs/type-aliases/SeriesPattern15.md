[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern15

# Type Alias: SeriesPattern15\<T\>

> **SeriesPattern15**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1565](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L1565)

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
