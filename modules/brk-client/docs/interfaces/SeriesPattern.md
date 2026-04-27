[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern

# Interface: SeriesPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1579](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L1579)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1581](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L1581)

Index endpoints as lazy getters

***

### get

> **get**: (`index`) => [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1583](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L1583)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

***

### indexes

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1582](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L1582)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1580](https://github.com/bitcoinresearchkit/brk/blob/76869ed2b6aad1e1c3da7aff4c90e9b5788fb606/modules/brk-client/index.js#L1580)

The series name
