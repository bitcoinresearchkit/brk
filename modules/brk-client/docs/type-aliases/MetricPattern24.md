[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern24

# Type Alias: MetricPattern24\<T\>

> **MetricPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1203](https://github.com/bitcoinresearchkit/brk/blob/616a97d242fb82081d065cfef39ccee8acd45c28/modules/brk-client/index.js#L1203)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2wshaddressindex

> `readonly` **p2wshaddressindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

### get()

> **get**: (`index`) => [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\> \| `undefined`

#### Parameters

##### index

[`Index`](Index.md)

#### Returns

[`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\> \| `undefined`

### indexes()

> **indexes**: () => readonly [`Index`](Index.md)[]

#### Returns

readonly [`Index`](Index.md)[]

### name

> **name**: `string`
