[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern

# Interface: SeriesPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1245](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1245)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1247](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1247)

Index endpoints as lazy getters

***

### get

> **get**: (`index`) => [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1249](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1249)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

***

### indexes

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1248](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1248)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1246](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1246)

The series name
