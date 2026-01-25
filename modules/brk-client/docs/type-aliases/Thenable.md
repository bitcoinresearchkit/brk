[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Thenable

# Type Alias: Thenable()\<T\>

> **Thenable**\<`T`\> = (`onfulfilled?`, `onrejected?`) => `Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:854](https://github.com/bitcoinresearchkit/brk/blob/1e76e137abea038ab55fde16571abe1d31a780c4/modules/brk-client/index.js#L854)

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
