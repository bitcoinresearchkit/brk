[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern27

# Type Alias: MetricPattern27\<T\>

> **MetricPattern27**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1199](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L1199)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.txindex

> `readonly` **txindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
