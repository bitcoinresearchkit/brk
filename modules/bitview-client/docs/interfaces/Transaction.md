[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / Transaction

# Interface: Transaction

Defined in: [Developer/mono/modules/bitview-client/index.js:1247](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1247)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1257](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1257)

Transaction fee in satoshis

***

### index?

> `optional` **index?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1248](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1248)

Internal transaction index (brk-specific, not in mempool.space)

***

### locktime

> **locktime**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1251](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1251)

Transaction lock time

***

### sigops

> **sigops**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1256](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1256)

Number of signature operations

***

### size

> **size**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1254](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1254)

Transaction size in bytes

***

### status

> **status**: [`TxStatus`](TxStatus.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1258](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1258)

Confirmation status (confirmed, block height/hash/time)

***

### txid

> **txid**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1249](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1249)

Transaction ID

***

### version

> **version**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1250](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1250)

Transaction version (raw i32 from Bitcoin protocol, may contain non-standard values in coinbase txs)

***

### vin

> **vin**: [`TxIn`](TxIn.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1252](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1252)

Transaction inputs

***

### vout

> **vout**: [`TxOut`](TxOut.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1253](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1253)

Transaction outputs

***

### weight

> **weight**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1255](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1255)

Transaction weight
