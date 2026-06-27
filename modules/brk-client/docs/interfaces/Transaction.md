[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Transaction

# Interface: Transaction

Defined in: [Developer/brk/modules/brk-client/index.js:1229](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1229)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1239](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1239)

Transaction fee in satoshis

***

### index?

> `optional` **index?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1230](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1230)

Internal transaction index (brk-specific, not in mempool.space)

***

### locktime

> **locktime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1233](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1233)

Transaction lock time

***

### sigops

> **sigops**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1238](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1238)

Number of signature operations

***

### size

> **size**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1236](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1236)

Transaction size in bytes

***

### status

> **status**: [`TxStatus`](TxStatus.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1240](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1240)

Confirmation status (confirmed, block height/hash/time)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1231](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1231)

Transaction ID

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1232](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1232)

Transaction version (raw i32 from Bitcoin protocol, may contain non-standard values in coinbase txs)

***

### vin

> **vin**: [`TxIn`](TxIn.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1234](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1234)

Transaction inputs

***

### vout

> **vout**: [`TxOut`](TxOut.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1235](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1235)

Transaction outputs

***

### weight

> **weight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1237](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L1237)

Transaction weight
