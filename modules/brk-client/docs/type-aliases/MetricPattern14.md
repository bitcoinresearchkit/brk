[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern14

# Type Alias: MetricPattern14\<T\>

> **MetricPattern14**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1173](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L1173)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.opreturnindex

> `readonly` **opreturnindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
