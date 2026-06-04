[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolBlock

# Interface: MempoolBlock

Defined in: [Developer/brk/modules/brk-client/index.js:731](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L731)

## Properties

### blockSize

> **blockSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:732](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L732)

Total serialized block size in bytes (witness + non-witness).

***

### blockVSize

> **blockVSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:733](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L733)

Total block virtual size in vbytes

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:737](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L737)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:736](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L736)

Median fee rate in sat/vB

***

### nTx

> **nTx**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:734](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L734)

Number of transactions in the projected block

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:735](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L735)

Total fees in satoshis
