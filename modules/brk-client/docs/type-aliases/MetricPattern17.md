[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern17

# Type Alias: MetricPattern17\<T\>

> **MetricPattern17**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1326](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L1326)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.p2msoutputindex

> `readonly` **p2msoutputindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
