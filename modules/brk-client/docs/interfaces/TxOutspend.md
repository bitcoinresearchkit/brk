[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:1047](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1047)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1048](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1048)

Whether the output has been spent

***

### status?

> `optional` **status?**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1051](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1051)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1049](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1049)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1050](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1050)

Input index in the spending transaction (only present if spent)
