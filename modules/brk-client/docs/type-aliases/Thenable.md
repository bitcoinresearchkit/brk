[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Thenable

# Type Alias: Thenable()\<T\>

> **Thenable**\<`T`\> = (`onfulfilled?`, `onrejected?`) => `Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:961](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L961)

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
