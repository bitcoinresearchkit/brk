[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern5

# Type Alias: SeriesPattern5\<T\>

> **SeriesPattern5**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:2211](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L2211)

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
