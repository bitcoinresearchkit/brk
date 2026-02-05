[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern10

# Type Alias: MetricPattern10\<T\>

> **MetricPattern10**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1330](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L1330)

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
