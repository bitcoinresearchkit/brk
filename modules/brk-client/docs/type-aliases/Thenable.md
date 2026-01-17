[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Thenable

# Type Alias: Thenable()\<T\>

> **Thenable**\<`T`\> = (`onfulfilled?`, `onrejected?`) => `Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:852](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L852)

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
