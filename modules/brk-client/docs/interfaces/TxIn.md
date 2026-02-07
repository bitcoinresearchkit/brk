[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxIn

# Interface: TxIn

Defined in: [Developer/brk/modules/brk-client/index.js:762](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L762)

## Properties

### innerRedeemscriptAsm?

> `optional` **innerRedeemscriptAsm**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:770](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L770)

Inner redeemscript in assembly format (for P2SH-wrapped SegWit)

***

### isCoinbase

> **isCoinbase**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:768](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L768)

Whether this input is a coinbase (block reward) input

***

### prevout?

> `optional` **prevout**: [`TxOut`](TxOut.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:765](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L765)

Information about the previous output being spent

***

### scriptsig

> **scriptsig**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:766](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L766)

Signature script (for non-SegWit inputs)

***

### scriptsigAsm

> **scriptsigAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:767](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L767)

Signature script in assembly format

***

### sequence

> **sequence**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:769](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L769)

Input sequence number

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:763](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L763)

Transaction ID of the output being spent

***

### vout

> **vout**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:764](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L764)
