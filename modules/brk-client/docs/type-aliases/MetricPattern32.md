[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern32

# Type Alias: MetricPattern32\<T\>

> **MetricPattern32**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1225](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L1225)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.emptyaddressindex

> `readonly` **emptyaddressindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
