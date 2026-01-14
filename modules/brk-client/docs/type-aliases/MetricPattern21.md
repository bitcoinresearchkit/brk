[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern21

# Type Alias: MetricPattern21\<T\>

> **MetricPattern21**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1181](https://github.com/bitcoinresearchkit/brk/blob/8a938c00f6edf1f447532c02f94f3a13fd7da30e/modules/brk-client/index.js#L1181)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2shaddressindex

> `readonly` **p2shaddressindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
