[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern16

# Type Alias: MetricPattern16\<T\>

> **MetricPattern16**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1166](https://github.com/bitcoinresearchkit/brk/blob/8a938c00f6edf1f447532c02f94f3a13fd7da30e/modules/brk-client/index.js#L1166)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2aaddressindex

> `readonly` **p2aaddressindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
