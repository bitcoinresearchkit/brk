[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern23

# Type Alias: MetricPattern23\<T\>

> **MetricPattern23**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1369](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L1369)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2wpkhaddressindex

> `readonly` **p2wpkhaddressindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
