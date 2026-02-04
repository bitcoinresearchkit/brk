[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern26

# Type Alias: MetricPattern26\<T\>

> **MetricPattern26**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1378](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1378)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.semesterindex

> `readonly` **semesterindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
