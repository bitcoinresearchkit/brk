[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern4

# Type Alias: MetricPattern4\<T\>

> **MetricPattern4**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1130](https://github.com/bitcoinresearchkit/brk/blob/8a938c00f6edf1f447532c02f94f3a13fd7da30e/modules/brk-client/index.js#L1130)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.dateindex

> `readonly` **dateindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

#### by.decadeindex

> `readonly` **decadeindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

#### by.monthindex

> `readonly` **monthindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

#### by.quarterindex

> `readonly` **quarterindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

#### by.semesterindex

> `readonly` **semesterindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

#### by.weekindex

> `readonly` **weekindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

#### by.yearindex

> `readonly` **yearindex**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
