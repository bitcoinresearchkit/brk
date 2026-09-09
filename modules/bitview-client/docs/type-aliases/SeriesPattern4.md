[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / SeriesPattern4

# Type Alias: SeriesPattern4\<T\>

> **SeriesPattern4**\<`T`\> = `object`

Defined in: [Developer/mono/modules/bitview-client/index.js:2433](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L2433)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.minute30

> `readonly` **minute30**: [`DateSeriesEndpoint`](../interfaces/DateSeriesEndpoint.md)\<`T`\>

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
