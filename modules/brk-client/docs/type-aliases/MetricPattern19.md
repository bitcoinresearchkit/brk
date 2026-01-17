[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern19

# Type Alias: MetricPattern19\<T\>

> **MetricPattern19**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1186](https://github.com/bitcoinresearchkit/brk/blob/64b90dd67834dd5b6acaff4f9a227efc18ccfed0/modules/brk-client/index.js#L1186)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2pk65addressindex

> `readonly` **p2pk65addressindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
