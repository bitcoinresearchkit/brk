[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern

# Interface: MetricPattern\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:565](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L565)

## Type Parameters

### T

`T`

## Properties

### by

> **by**: `Partial`\<`Record`\<[`Index`](../type-aliases/Index.md), [`MetricEndpoint`](MetricEndpoint.md)\<`T`\>\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:567](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L567)

Index endpoints (lazy getters)

***

### get()

> **get**: (`index`) => [`MetricEndpoint`](MetricEndpoint.md)\<`T`\> \| `undefined`

Defined in: [Developer/brk/modules/brk-client/index.js:569](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L569)

Get an endpoint for a specific index

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

[`MetricEndpoint`](MetricEndpoint.md)\<`T`\> \| `undefined`

***

### indexes()

> **indexes**: () => [`Index`](../type-aliases/Index.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:568](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L568)

Get the list of available indexes

#### Returns

[`Index`](../type-aliases/Index.md)[]

***

### name

> **name**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:566](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L566)

The metric name
