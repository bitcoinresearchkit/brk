[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / MempoolBlock

# Interface: MempoolBlock

Defined in: [Developer/mono/modules/bitview-client/index.js:717](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L717)

## Properties

### blockSize

> **blockSize**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:718](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L718)

Total serialized block size in bytes (witness + non-witness).

***

### blockVSize

> **blockVSize**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:719](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L719)

Total block virtual size in vbytes

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:723](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L723)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:722](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L722)

Median fee rate in sat/vB

***

### nTx

> **nTx**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:720](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L720)

Number of transactions in the projected block

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:721](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L721)

Total fees in satoshis
