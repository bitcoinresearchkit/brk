[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern11

# Type Alias: MetricPattern11\<T\>

> **MetricPattern11**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1333](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L1333)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.height

> `readonly` **height**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
