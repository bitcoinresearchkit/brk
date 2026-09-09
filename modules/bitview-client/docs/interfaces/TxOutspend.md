[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/mono/modules/bitview-client/index.js:1305](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1305)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1306](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1306)

Whether the output has been spent

***

### status?

> `optional` **status?**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1309](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1309)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid?**: `string` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1307](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1307)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1308](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1308)

Input index in the spending transaction (only present if spent)
