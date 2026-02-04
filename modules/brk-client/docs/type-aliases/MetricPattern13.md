[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern13

# Type Alias: MetricPattern13\<T\>

> **MetricPattern13**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1339](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1339)

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
