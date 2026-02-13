[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Thenable

# Type Alias: Thenable()\<T\>

> **Thenable**\<`T`\> = (`onfulfilled?`, `onrejected?`) => `Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1024](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1024)

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
