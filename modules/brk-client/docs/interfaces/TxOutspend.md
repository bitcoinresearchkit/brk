[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:1274](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L1274)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1275](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L1275)

Whether the output has been spent

***

### status?

> `optional` **status?**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1278](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L1278)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1276](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L1276)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1277](https://github.com/bitcoinresearchkit/brk/blob/c85da92cbcb3b8645b12f2986df719688adb8dff/modules/brk-client/index.js#L1277)

Input index in the spending transaction (only present if spent)
