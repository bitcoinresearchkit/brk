[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxIn

# Interface: TxIn

Defined in: [Developer/brk/modules/brk-client/index.js:687](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L687)

## Properties

### innerRedeemscriptAsm?

> `optional` **innerRedeemscriptAsm**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:695](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L695)

Inner redeemscript in assembly format (for P2SH-wrapped SegWit)

***

### isCoinbase

> **isCoinbase**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:693](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L693)

Whether this input is a coinbase (block reward) input

***

### prevout?

> `optional` **prevout**: [`TxOut`](TxOut.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:690](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L690)

Information about the previous output being spent

***

### scriptsig

> **scriptsig**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:691](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L691)

Signature script (for non-SegWit inputs)

***

### scriptsigAsm

> **scriptsigAsm**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:692](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L692)

Signature script in assembly format

***

### sequence

> **sequence**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:694](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L694)

Input sequence number

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:688](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L688)

Transaction ID of the output being spent

***

### vout

> **vout**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:689](https://github.com/bitcoinresearchkit/brk/blob/467dfcc4b801a25e7d5f0ec4af8dddfffc3861d4/modules/brk-client/index.js#L689)
