[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern26

# Type Alias: MetricPattern26\<T\>

> **MetricPattern26**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1207](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L1207)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.semesterindex

> `readonly` **semesterindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
