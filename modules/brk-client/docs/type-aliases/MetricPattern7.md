[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern7

# Type Alias: MetricPattern7\<T\>

> **MetricPattern7**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1359](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L1359)

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
