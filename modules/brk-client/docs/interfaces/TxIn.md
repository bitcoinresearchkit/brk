[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxIn

# Interface: TxIn

Defined in: [Developer/brk/modules/brk-client/index.js:1095](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1095)

## Properties

### innerRedeemscriptAsm

> **innerRedeemscriptAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1104](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1104)

Inner redeemscript in assembly (for P2SH-wrapped SegWit: scriptsig + witness both present)

***

### innerWitnessscriptAsm

> **innerWitnessscriptAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1105](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1105)

Inner witnessscript in assembly (for P2WSH: last witness item decoded as script)

***

### isCoinbase

> **isCoinbase**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1102](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1102)

Whether this input is a coinbase (block reward) input

***

### prevout?

> `optional` **prevout?**: [`TxOut`](TxOut.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1098](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1098)

Information about the previous output being spent

***

### scriptsig

> **scriptsig**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1099](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1099)

Signature script (hex, for non-SegWit inputs)

***

### scriptsigAsm

> **scriptsigAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1100](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1100)

Signature script in assembly format

***

### sequence

> **sequence**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1103](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1103)

Input sequence number

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1096](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1096)

Transaction ID of the output being spent

***

### vout

> **vout**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1097](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1097)

Output index being spent (u16: coinbase is 65535, mempool.space uses u32: 4294967295)

***

### witness

> **witness**: [`Witness`](../type-aliases/Witness.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1101](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1101)

Witness data (stack items, present for SegWit inputs; hex-encoded on the wire)
