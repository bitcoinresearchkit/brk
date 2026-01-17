[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern2

# Type Alias: MetricPattern2\<T\>

> **MetricPattern2**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:1135](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L1135)

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

#### by.difficultyepoch

> `readonly` **difficultyepoch**: [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`T`\>

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
