[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / TxIn

# Interface: TxIn

Defined in: [Developer/mono/modules/bitview-client/index.js:1268](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1268)

## Properties

### innerRedeemscriptAsm?

> `optional` **innerRedeemscriptAsm?**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1277](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1277)

Inner redeemscript in assembly (for P2SH-wrapped SegWit: scriptsig + witness both present)

***

### innerWitnessscriptAsm?

> `optional` **innerWitnessscriptAsm?**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1278](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1278)

Inner witnessscript in assembly (for P2WSH: last witness item decoded as script)

***

### isCoinbase

> **isCoinbase**: `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1275](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1275)

Whether this input is a coinbase (block reward) input

***

### prevout

> **prevout**: [`TxOut`](TxOut.md) \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1271](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1271)

Information about the previous output being spent

***

### scriptsig

> **scriptsig**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1272](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1272)

Signature script (hex, for non-SegWit inputs)

***

### scriptsigAsm

> **scriptsigAsm**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1273](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1273)

Signature script in assembly format

***

### sequence

> **sequence**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1276](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1276)

Input sequence number

***

### txid

> **txid**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1269](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1269)

Transaction ID of the output being spent

***

### vout

> **vout**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1270](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1270)

Output index being spent (u16: coinbase is 65535, mempool.space uses u32: 4294967295)

***

### witness?

> `optional` **witness?**: [`Witness`](../type-aliases/Witness.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1274](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1274)

Witness data (stack items, present for SegWit inputs; hex-encoded on the wire)
