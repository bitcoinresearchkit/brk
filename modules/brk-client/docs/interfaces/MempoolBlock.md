[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolBlock

# Interface: MempoolBlock

Defined in: [Developer/brk/modules/brk-client/index.js:628](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L628)

## Properties

### blockSize

> **blockSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:629](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L629)

Total serialized block size in bytes (witness + non-witness).

***

### blockVSize

> **blockVSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:630](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L630)

Total block virtual size in vbytes

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:634](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L634)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:633](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L633)

Median fee rate in sat/vB

***

### nTx

> **nTx**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:631](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L631)

Number of transactions in the projected block

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:632](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L632)

Total fees in satoshis
