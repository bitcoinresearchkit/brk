[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern

# Interface: MetricPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:900](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L900)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Readonly`\<`Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\>\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:902](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L902)

Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']

***

### get()

> **get**: (`index`) => [`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:904](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L904)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`MetricEndpointBuilder`](MetricEndpointBuilder.md)\<`T`\> \| `undefined`

***

### indexes()

> **indexes**: () => readonly [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:903](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L903)

Get the list of available indexes

#### Returns

readonly [`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:901](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L901)

The metric name
