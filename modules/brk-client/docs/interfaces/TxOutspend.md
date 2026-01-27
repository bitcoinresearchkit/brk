[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:722](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L722)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:723](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L723)

Whether the output has been spent

***

### status?

> `optional` **status**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:726](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L726)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:724](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L724)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:725](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L725)

Input index in the spending transaction (only present if spent)
