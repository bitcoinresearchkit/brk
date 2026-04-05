[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Transaction

# Interface: Transaction

Defined in: [Developer/brk/modules/brk-client/index.js:1021](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1021)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1031](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1031)

Transaction fee in satoshis

***

### index?

> `optional` **index?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1022](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1022)

Internal transaction index (brk-specific, not in mempool.space)

***

### locktime

> **locktime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1025](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1025)

Transaction lock time

***

### sigops

> **sigops**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1030](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1030)

Number of signature operations

***

### size

> **size**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1028](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1028)

Transaction size in bytes

***

### status

> **status**: [`TxStatus`](TxStatus.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1032](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1032)

Confirmation status (confirmed, block height/hash/time)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1023](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1023)

Transaction ID

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1024](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1024)

Transaction version (raw i32 from Bitcoin protocol, may contain non-standard values in coinbase txs)

***

### vin

> **vin**: [`TxIn`](TxIn.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1026](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1026)

Transaction inputs

***

### vout

> **vout**: [`TxOut`](TxOut.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1027](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1027)

Transaction outputs

***

### weight

> **weight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1029](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1029)

Transaction weight
