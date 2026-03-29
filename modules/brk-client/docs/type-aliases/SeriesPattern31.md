[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern31

# Type Alias: SeriesPattern31\<T\>

> **SeriesPattern31**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1613](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L1613)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2wpkh\_addr\_index

> `readonly` **p2wpkh\_addr\_index**: [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`T`\>

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
