[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern

# Interface: SeriesPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1740](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1740)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1742](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1742)

Index endpoints as lazy getters

***

### get

> **get**: (`index`) => [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1744](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1744)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

***

### indexes

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1743](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1743)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1741](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L1741)

The series name
