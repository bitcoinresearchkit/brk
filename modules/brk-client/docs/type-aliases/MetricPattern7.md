[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern7

# Type Alias: MetricPattern7\<T\>

> **MetricPattern7**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1153](https://github.com/bitcoinresearchkit/brk/blob/ec1f2de5cfe92fbc8dc9dc980bbf7787003b4572/modules/brk-client/index.js#L1153)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.decadeindex

> `readonly` **decadeindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
