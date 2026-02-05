[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern24

# Type Alias: MetricPattern24\<T\>

> **MetricPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1372](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L1372)

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
