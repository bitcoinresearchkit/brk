[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockExtras

# Interface: BlockExtras

Defined in: [Developer/brk/modules/brk-client/index.js:136](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L136)

## Properties

### avgFee

> **avgFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:142](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L142)

Average fee per transaction in satoshis

***

### avgFeeRate

> **avgFeeRate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:143](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L143)

Average fee rate in sat/vB

***

### avgTxSize

> **avgTxSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:149](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L149)

Average transaction size in bytes

***

### coinbaseAddress?

> `optional` **coinbaseAddress?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:145](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L145)

Primary coinbase output address

***

### coinbaseAddresses

> **coinbaseAddresses**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:146](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L146)

All coinbase output addresses

***

### coinbaseRaw

> **coinbaseRaw**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:144](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L144)

Raw coinbase transaction scriptsig as hex

***

### coinbaseSignature

> **coinbaseSignature**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:147](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L147)

Coinbase output script in ASM format

***

### coinbaseSignatureAscii

> **coinbaseSignatureAscii**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:148](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L148)

Coinbase scriptsig decoded as ASCII

***

### feePercentiles

> **feePercentiles**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:154](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L154)

Fee amount percentiles in satoshis: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:139](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L139)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### firstSeen?

> `optional` **firstSeen?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:165](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L165)

Timestamp when the block was first seen (always null, not yet supported)

***

### header

> **header**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:158](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L158)

Raw 80-byte block header as hex

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:138](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L138)

Median fee rate in sat/vB

***

### medianFeeAmt

> **medianFeeAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:153](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L153)

Median fee amount in satoshis

***

### orphans

> **orphans**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:166](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L166)

Orphaned blocks (always empty)

***

### pool

> **pool**: [`BlockPool`](BlockPool.md)

Defined in: [Developer/brk/modules/brk-client/index.js:141](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L141)

Mining pool that mined this block

***

### price

> **price**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:167](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L167)

USD price at block height

***

### reward

> **reward**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:140](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L140)

Total block reward (subsidy + fees) in satoshis

***

### segwitTotalSize

> **segwitTotalSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:156](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L156)

Total size of segwit transactions in bytes

***

### segwitTotalTxs

> **segwitTotalTxs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:155](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L155)

Number of segwit transactions

***

### segwitTotalWeight

> **segwitTotalWeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:157](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L157)

Total weight of segwit transactions

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:137](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L137)

Total fees in satoshis

***

### totalInputAmt

> **totalInputAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:163](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L163)

Total input amount in satoshis

***

### totalInputs

> **totalInputs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:150](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L150)

Total number of inputs (excluding coinbase)

***

### totalOutputAmt

> **totalOutputAmt**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:152](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L152)

Total output amount in satoshis

***

### totalOutputs

> **totalOutputs**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:151](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L151)

Total number of outputs

***

### utxoSetChange

> **utxoSetChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:159](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L159)

UTXO set change (total outputs - total inputs, includes unspendable like OP_RETURN).
Note: intentionally differs from utxo_set_size diff which excludes unspendable outputs.
Matches mempool.space/bitcoin-cli behavior.

***

### utxoSetSize

> **utxoSetSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:162](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L162)

Total spendable UTXO set size at this height (excludes OP_RETURN and other unspendable outputs)

***

### virtualSize

> **virtualSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:164](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L164)

Virtual size in vbytes
