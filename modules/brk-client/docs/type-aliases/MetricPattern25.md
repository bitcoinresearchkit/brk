[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern25

# Type Alias: MetricPattern25\<T\>

> **MetricPattern25**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1193](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L1193)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.quarterindex

> `readonly` **quarterindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
