[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern

# Interface: MetricPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1007](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L1007)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1009](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L1009)

Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']

***

### get()

> **get**: (`index`) => [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1011](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L1011)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

***

### indexes()

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1010](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L1010)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1008](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L1008)

The metric name
