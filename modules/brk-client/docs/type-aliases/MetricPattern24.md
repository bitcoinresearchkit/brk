[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern24

# Type Alias: MetricPattern24\<T\>

> **MetricPattern24**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1201](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L1201)

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
