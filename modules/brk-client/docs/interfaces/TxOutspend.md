[**brk-client**](../README.md)

***

[brk-client](../globals.md) / TxOutspend

# Interface: TxOutspend

Defined in: [Developer/brk/modules/brk-client/index.js:785](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L785)

## Properties

### spent

> **spent**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:786](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L786)

Whether the output has been spent

***

### status?

> `optional` **status**: [`TxStatus`](TxStatus.md) \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:789](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L789)

Status of the spending transaction (only present if spent)

***

### txid?

> `optional` **txid**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:787](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L787)

Transaction ID of the spending transaction (only present if spent)

***

### vin?

> `optional` **vin**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:788](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L788)

Input index in the spending transaction (only present if spent)
