[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern6

# Type Alias: MetricPattern6\<T\>

> **MetricPattern6**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1136](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L1136)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.dateindex

> `readonly` **dateindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
