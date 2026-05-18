[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolBlock

# Interface: MempoolBlock

Defined in: [Developer/brk/modules/brk-client/index.js:723](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L723)

## Properties

### blockSize

> **blockSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:724](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L724)

Total serialized block size in bytes (witness + non-witness).

***

### blockVSize

> **blockVSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:725](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L725)

Total block virtual size in vbytes

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:729](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L729)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:728](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L728)

Median fee rate in sat/vB

***

### nTx

> **nTx**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:726](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L726)

Number of transactions in the projected block

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:727](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L727)

Total fees in satoshis
