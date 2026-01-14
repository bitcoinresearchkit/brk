[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern

# Interface: MetricPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:888](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L888)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:890](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L890)

Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']

***

### get()

> **get**: (`index`) => [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:892](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L892)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

***

### indexes()

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:891](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L891)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:889](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L889)

The metric name
