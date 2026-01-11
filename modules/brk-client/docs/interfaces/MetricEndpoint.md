[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricEndpoint

# Interface: MetricEndpoint\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:556](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L556)

## Type Parameters

### T

`T`

## Properties

### get()

> **get**: (`onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:557](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L557)

Fetch all data points

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:559](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L559)

The endpoint path

***

### range()

> **range**: (`from?`, `to?`, `onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:558](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L558)

Fetch data in range

#### Parameters

##### from?

`number`

##### to?

`number`

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>
