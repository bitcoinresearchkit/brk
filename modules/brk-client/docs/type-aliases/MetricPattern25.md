[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern25

# Type Alias: MetricPattern25\<T\>

> **MetricPattern25**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1413](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1413)

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
