[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricPattern4

# Type Alias: MetricPattern4\<T\>

> **MetricPattern4**\<`T`\> = `object`

Defined in: [Developer/brk/modules/brk-client/index.js:853](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L853)

## Type Parameters

### T

`T`

## Type Declaration

### by

> **by**: `object`

#### by.dateindex

> **dateindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

#### by.decadeindex

> **decadeindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

#### by.monthindex

> **monthindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

#### by.quarterindex

> **quarterindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

#### by.semesterindex

> **semesterindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

#### by.weekindex

> **weekindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

#### by.yearindex

> **yearindex**: [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\>

### get()

> **get**: (`index`) => [`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\> \| `undefined`

#### Parameters

##### index

[`Index`](Index.md)

#### Returns

[`MetricEndpoint`](../interfaces/MetricEndpoint.md)\<`T`\> \| `undefined`

### indexes()

> **indexes**: () => [`Index`](Index.md)[]

#### Returns

[`Index`](Index.md)[]

### name

> **name**: `string`
