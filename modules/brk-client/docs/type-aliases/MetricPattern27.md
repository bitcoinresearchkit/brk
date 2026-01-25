[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern27

# Type Alias: MetricPattern27\<T\>

> **MetricPattern27**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1212](https://github.com/bitcoinresearchkit/brk/blob/1e76e137abea038ab55fde16571abe1d31a780c4/modules/brk-client/index.js#L1212)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.txindex

> `readonly` **txindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
