[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern15

# Type Alias: MetricPattern15\<T\>

> **MetricPattern15**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1383](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L1383)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.txoutindex

> `readonly` **txoutindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
