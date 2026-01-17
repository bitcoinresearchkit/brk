[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RangeBuilder

# Interface: RangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:890](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L890)

## Type Parameters

### T

`T`

## Properties

### fetch()

> **fetch**: (`onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:891](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L891)

Fetch the range

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

***

### fetchCsv()

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:892](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L892)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:893](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L893)

Thenable
