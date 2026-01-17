[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxIn

# Interface: TxIn

Defined in: [Developer/brk/modules/brk-client/index.js:697](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L697)

## Properties

### innerRedeemscriptAsm?

> `optional` **innerRedeemscriptAsm**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:705](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L705)

Inner redeemscript in assembly format (for P2SH-wrapped SegWit)

***

### isCoinbase

> **isCoinbase**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:703](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L703)

Whether this input is a coinbase (block reward) input

***

### prevout?

> `optional` **prevout**: [`TxOut`](TxOut.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:700](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L700)

Information about the previous output being spent

***

### scriptsig

> **scriptsig**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:701](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L701)

Signature script (for non-SegWit inputs)

***

### scriptsigAsm

> **scriptsigAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:702](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L702)

Signature script in assembly format

***

### sequence

> **sequence**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:704](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L704)

Input sequence number

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:698](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L698)

Transaction ID of the output being spent

***

### vout

> **vout**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:699](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L699)
