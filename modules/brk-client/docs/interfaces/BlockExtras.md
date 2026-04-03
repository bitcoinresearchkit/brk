[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockExtras

# Interface: BlockExtras

Defined in: [Developer/brk/modules/brk-client/index.js:115](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L115)

## Properties

### avgFee

> **avgFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:121](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L121)

Average fee per transaction in satoshis

***

### avgFeeRate

> **avgFeeRate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:122](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L122)

Average fee rate in sat/vB

***

### avgTxSize

> **avgTxSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:128](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L128)

Average transaction size in bytes

***

### coinbaseAddress?

> `optional` **coinbaseAddress?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:124](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L124)

Primary coinbase output address

***

### coinbaseAddresses

> **coinbaseAddresses**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:125](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L125)

All coinbase output addresses

***

### coinbaseRaw

> **coinbaseRaw**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:123](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L123)

Raw coinbase transaction scriptsig as hex

***

### coinbaseSignature

> **coinbaseSignature**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:126](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L126)

Coinbase output script in ASM format

***

### coinbaseSignatureAscii

> **coinbaseSignatureAscii**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:127](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L127)

Coinbase scriptsig decoded as ASCII

***

### feePercentiles

> **feePercentiles**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:133](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L133)

Fee amount percentiles in satoshis: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:118](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L118)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### header

> **header**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:137](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L137)

Raw 80-byte block header as hex

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:117](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L117)

Median fee rate in sat/vB

***

### medianFeeAmt

> **medianFeeAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:132](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L132)

Median fee amount in satoshis

***

### pool

> **pool**: [`BlockPool`](BlockPool.md)

Defined in: [Developer/brk/modules/brk-client/index.js:120](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L120)

Mining pool that mined this block

***

### price

> **price**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:142](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L142)

USD price at block height

***

### reward

> **reward**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:119](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L119)

Total block reward (subsidy + fees) in satoshis

***

### segwitTotalSize

> **segwitTotalSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:135](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L135)

Total size of segwit transactions in bytes

***

### segwitTotalTxs

> **segwitTotalTxs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:134](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L134)

Number of segwit transactions

***

### segwitTotalWeight

> **segwitTotalWeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:136](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L136)

Total weight of segwit transactions

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:116](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L116)

Total fees in satoshis

***

### totalInputAmt

> **totalInputAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:140](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L140)

Total input amount in satoshis

***

### totalInputs

> **totalInputs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:129](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L129)

Total number of inputs (excluding coinbase)

***

### totalOutputAmt

> **totalOutputAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:131](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L131)

Total output amount in satoshis

***

### totalOutputs

> **totalOutputs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:130](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L130)

Total number of outputs

***

### utxoSetChange

> **utxoSetChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:138](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L138)

UTXO set change (outputs created minus inputs spent)

***

### utxoSetSize

> **utxoSetSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:139](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L139)

Total UTXO set size at this height

***

### virtualSize

> **virtualSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:141](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L141)

Virtual size in vbytes
