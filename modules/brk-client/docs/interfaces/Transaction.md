[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Transaction

# Interface: Transaction

Defined in: [Developer/brk/modules/brk-client/index.js:1217](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1217)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1227](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1227)

Transaction fee in satoshis

***

### index?

> `optional` **index?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1218](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1218)

Internal transaction index (brk-specific, not in mempool.space)

***

### locktime

> **locktime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1221](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1221)

Transaction lock time

***

### sigops

> **sigops**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1226](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1226)

Number of signature operations

***

### size

> **size**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1224](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1224)

Transaction size in bytes

***

### status

> **status**: [`TxStatus`](TxStatus.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1228](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1228)

Confirmation status (confirmed, block height/hash/time)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1219](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1219)

Transaction ID

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1220](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1220)

Transaction version (raw i32 from Bitcoin protocol, may contain non-standard values in coinbase txs)

***

### vin

> **vin**: [`TxIn`](TxIn.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1222](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1222)

Transaction inputs

***

### vout

> **vout**: [`TxOut`](TxOut.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1223](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1223)

Transaction outputs

***

### weight

> **weight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1225](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1225)

Transaction weight
