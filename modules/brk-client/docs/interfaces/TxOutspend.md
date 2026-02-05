[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:747](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L747)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:748](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L748)

Whether the output has been spent

***

### status?

> `optional` **status**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:751](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L751)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:749](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L749)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:750](https://github.com/bitcoinresearchkit/brk/blob/dc15cceb1ee5fd90210add64f487f8d2140ef45a/modules/brk-client/index.js#L750)

Input index in the spending transaction (only present if spent)
