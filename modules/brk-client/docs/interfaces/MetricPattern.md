[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern

# Interface: MetricPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1032](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L1032)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1034](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L1034)

Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']

***

### get()

> **get**: (`index`) => [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1036](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L1036)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

***

### indexes()

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1035](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L1035)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1033](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L1033)

The metric name
