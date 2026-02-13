[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern29

# Type Alias: MetricPattern29\<T\>

> **MetricPattern29**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1425](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1425)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.weekindex

> `readonly` **weekindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
