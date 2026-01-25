[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern13

# Type Alias: MetricPattern13\<T\>

> **MetricPattern13**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1170](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L1170)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.monthindex

> `readonly` **monthindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
