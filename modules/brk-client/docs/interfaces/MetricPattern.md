[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern

# Interface: MetricPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1070](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1070)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1072](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1072)

Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']

***

### get()

> **get**: (`index`) => [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:1074](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1074)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

***

### indexes()

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1073](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1073)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1071](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1071)

The metric name
