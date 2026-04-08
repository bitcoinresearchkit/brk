[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxIn

# Interface: TxIn

Defined in: [Developer/brk/modules/brk-client/index.js:1070](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1070)

## Properties

### innerRedeemscriptAsm

> **innerRedeemscriptAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1079](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1079)

Inner redeemscript in assembly (for P2SH-wrapped SegWit: scriptsig + witness both present)

***

### innerWitnessscriptAsm

> **innerWitnessscriptAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1080](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1080)

Inner witnessscript in assembly (for P2WSH: last witness item decoded as script)

***

### isCoinbase

> **isCoinbase**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1077](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1077)

Whether this input is a coinbase (block reward) input

***

### prevout?

> `optional` **prevout?**: [`TxOut`](TxOut.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1073](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1073)

Information about the previous output being spent

***

### scriptsig

> **scriptsig**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1074](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1074)

Signature script (hex, for non-SegWit inputs)

***

### scriptsigAsm

> **scriptsigAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1075](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1075)

Signature script in assembly format

***

### sequence

> **sequence**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1078](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1078)

Input sequence number

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1071](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1071)

Transaction ID of the output being spent

***

### vout

> **vout**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1072](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1072)

Output index being spent (u16: coinbase is 65535, mempool.space uses u32: 4294967295)

***

### witness

> **witness**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1076](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L1076)

Witness data (hex-encoded stack items, present for SegWit inputs)
