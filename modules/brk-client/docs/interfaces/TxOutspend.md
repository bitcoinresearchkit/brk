[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:1050](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1050)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1051](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1051)

Whether the output has been spent

***

### status?

> `optional` **status?**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1054](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1054)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1052](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1052)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1053](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1053)

Input index in the spending transaction (only present if spent)
