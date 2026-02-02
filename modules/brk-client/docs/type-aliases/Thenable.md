[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Thenable

# Type Alias: Thenable()\<T\>

> **Thenable**\<`T`\> = (`onfulfilled?`, `onrejected?`) => `Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:986](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L986)

## Type Parameters

### T

`T`

## Parameters

### onfulfilled?

(`value`) => [`MetricData`](../interfaces/MetricData.md)\<`T`\>

### onrejected?

(`reason`) => `never`

## Returns

`Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>
