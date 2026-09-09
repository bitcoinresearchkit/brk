[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / TxOut

# Interface: TxOut

Defined in: [Developer/mono/modules/bitview-client/index.js:1294](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1294)

## Properties

### scriptpubkey

> **scriptpubkey**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1295](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1295)

Script pubkey (locking script), encoded as hexadecimal.

***

### scriptpubkeyAddress?

> `optional` **scriptpubkeyAddress?**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1298](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1298)

Bitcoin address, omitted for scripts without an address.

***

### scriptpubkeyAsm

> **scriptpubkeyAsm**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1296](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1296)

Script pubkey in assembly format.

***

### scriptpubkeyType

> **scriptpubkeyType**: [`OutputTypeNormalized`](../type-aliases/OutputTypeNormalized.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1297](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1297)

Esplora/mempool.space script type.

***

### value

> **value**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1299](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1299)

Value of the output in satoshis.
