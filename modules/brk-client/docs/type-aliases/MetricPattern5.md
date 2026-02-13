[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern5

# Type Alias: MetricPattern5\<T\>

> **MetricPattern5**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1353](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1353)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.dateindex

> `readonly` **dateindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
