[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern10

# Type Alias: MetricPattern10\<T\>

> **MetricPattern10**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1162](https://github.com/bitcoinresearchkit/brk/blob/f6020b32a7a84b2a0789fb283897b90b97fc4836/modules/brk-client/index.js#L1162)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.halvingepoch

> `readonly` **halvingepoch**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
