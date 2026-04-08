[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesPattern

# Interface: SeriesPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1485](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1485)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1487](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1487)

Index endpoints as lazy getters

***

### get

> **get**: (`index`) => [`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1489](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1489)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`SeriesEndpoint`](SeriesEndpoint.md)\<`T`\> \| `undefined`

***

### indexes

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1488](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1488)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1486](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1486)

The series name
