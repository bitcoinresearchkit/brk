[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern12

# Type Alias: MetricPattern12\<T\>

> **MetricPattern12**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1168](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L1168)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.txinindex

> `readonly` **txinindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
