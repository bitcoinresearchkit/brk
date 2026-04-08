[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockExtras

# Interface: BlockExtras

Defined in: [Developer/brk/modules/brk-client/index.js:121](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L121)

## Properties

### avgFee

> **avgFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:127](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L127)

Average fee per transaction in satoshis

***

### avgFeeRate

> **avgFeeRate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:128](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L128)

Average fee rate in sat/vB

***

### avgTxSize

> **avgTxSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:134](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L134)

Average transaction size in bytes

***

### coinbaseAddress?

> `optional` **coinbaseAddress?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:130](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L130)

Primary coinbase output address

***

### coinbaseAddresses

> **coinbaseAddresses**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:131](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L131)

All coinbase output addresses

***

### coinbaseRaw

> **coinbaseRaw**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:129](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L129)

Raw coinbase transaction scriptsig as hex

***

### coinbaseSignature

> **coinbaseSignature**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:132](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L132)

Coinbase output script in ASM format

***

### coinbaseSignatureAscii

> **coinbaseSignatureAscii**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:133](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L133)

Coinbase scriptsig decoded as ASCII

***

### feePercentiles

> **feePercentiles**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:139](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L139)

Fee amount percentiles in satoshis: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:124](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L124)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### firstSeen?

> `optional` **firstSeen?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:150](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L150)

Timestamp when the block was first seen (always null, not yet supported)

***

### header

> **header**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:143](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L143)

Raw 80-byte block header as hex

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:123](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L123)

Median fee rate in sat/vB

***

### medianFeeAmt

> **medianFeeAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:138](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L138)

Median fee amount in satoshis

***

### orphans

> **orphans**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:151](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L151)

Orphaned blocks (always empty)

***

### pool

> **pool**: [`BlockPool`](BlockPool.md)

Defined in: [Developer/brk/modules/brk-client/index.js:126](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L126)

Mining pool that mined this block

***

### price

> **price**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:152](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L152)

USD price at block height

***

### reward

> **reward**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:125](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L125)

Total block reward (subsidy + fees) in satoshis

***

### segwitTotalSize

> **segwitTotalSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:141](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L141)

Total size of segwit transactions in bytes

***

### segwitTotalTxs

> **segwitTotalTxs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:140](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L140)

Number of segwit transactions

***

### segwitTotalWeight

> **segwitTotalWeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:142](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L142)

Total weight of segwit transactions

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:122](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L122)

Total fees in satoshis

***

### totalInputAmt

> **totalInputAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:148](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L148)

Total input amount in satoshis

***

### totalInputs

> **totalInputs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:135](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L135)

Total number of inputs (excluding coinbase)

***

### totalOutputAmt

> **totalOutputAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:137](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L137)

Total output amount in satoshis

***

### totalOutputs

> **totalOutputs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:136](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L136)

Total number of outputs

***

### utxoSetChange

> **utxoSetChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:144](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L144)

UTXO set change (total outputs - total inputs, includes unspendable like OP_RETURN).
Note: intentionally differs from utxo_set_size diff which excludes unspendable outputs.
Matches mempool.space/bitcoin-cli behavior.

***

### utxoSetSize

> **utxoSetSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:147](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L147)

Total spendable UTXO set size at this height (excludes OP_RETURN and other unspendable outputs)

***

### virtualSize

> **virtualSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:149](https://github.com/bitcoinresearchkit/brk/blob/1ddb3385e298de5498f1b3bf48bb88ed008d8e17/modules/brk-client/index.js#L149)

Virtual size in vbytes
