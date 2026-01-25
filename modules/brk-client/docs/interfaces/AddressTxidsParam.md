[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddressTxidsParam

# Interface: AddressTxidsParam

Defined in: [Developer/brk/modules/brk-client/index.js:49](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L49)

## Properties

### afterTxid?

> `optional` **afterTxid**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:50](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L50)

Txid to paginate from (return transactions before this one)

***

### limit?

> `optional` **limit**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:51](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L51)

Maximum number of results to return. Defaults to 25 if not specified.
