[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:1274](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1274)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1275](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1275)

Whether the output has been spent

***

### status?

> `optional` **status?**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1278](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1278)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1276](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1276)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1277](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1277)

Input index in the spending transaction (only present if spent)
