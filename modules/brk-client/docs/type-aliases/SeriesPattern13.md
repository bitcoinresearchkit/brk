[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern13

# Type Alias: SeriesPattern13\<T\>

> **SeriesPattern13**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2214](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L2214)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.month6

> `readonly` **month6**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
