[**brk-client**](../README.md)

***

[brk-client](../globals.md) / Transaction

# Interface: Transaction

Defined in: [Developer/brk/modules/brk-client/index.js:1074](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1074)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1084](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1084)

Transaction fee in satoshis

***

### index?

> `optional` **index?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1075](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1075)

Internal transaction index (brk-specific, not in mempool.space)

***

### locktime

> **locktime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1078](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1078)

Transaction lock time

***

### sigops

> **sigops**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1083](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1083)

Number of signature operations

***

### size

> **size**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1081](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1081)

Transaction size in bytes

***

### status

> **status**: [`TxStatus`](TxStatus.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1085](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1085)

Confirmation status (confirmed, block height/hash/time)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1076](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1076)

Transaction ID

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1077](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1077)

Transaction version (raw i32 from Bitcoin protocol, may contain non-standard values in coinbase txs)

***

### vin

> **vin**: [`TxIn`](TxIn.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1079](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1079)

Transaction inputs

***

### vout

> **vout**: [`TxOut`](TxOut.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:1080](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1080)

Transaction outputs

***

### weight

> **weight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1082](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1082)

Transaction weight
