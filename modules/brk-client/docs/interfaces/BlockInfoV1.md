[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockInfoV1

# Interface: BlockInfoV1

Defined in: [Developer/brk/modules/brk-client/index.js:190](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L190)

## Properties

### bits

> **bits**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:197](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L197)

Compact target (bits)

***

### difficulty

> **difficulty**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:204](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L204)

Block difficulty

***

### extras

> **extras**: [`BlockExtras`](BlockExtras.md)

Defined in: [Developer/brk/modules/brk-client/index.js:205](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L205)

Extended block data

***

### height

> **height**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:192](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L192)

Block height

***

### id

> **id**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:191](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L191)

Block hash

***

### mediantime

> **mediantime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:203](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L203)

Median time of the last 11 blocks

***

### merkleRoot

> **merkleRoot**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:195](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L195)

Merkle root of the transaction tree

***

### nonce

> **nonce**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:198](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L198)

Nonce used to produce a valid block hash

***

### previousblockhash

> **previousblockhash**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:194](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L194)

Previous block hash

***

### size

> **size**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:201](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L201)

Block size in bytes

***

### time

> **time**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:196](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L196)

Block timestamp as claimed by the miner (Unix time)

***

### timestamp

> **timestamp**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:199](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L199)

Block timestamp (Unix time)

***

### txCount

> **txCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:200](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L200)

Number of transactions in the block

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:193](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L193)

Block version, used for soft fork signaling

***

### weight

> **weight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:202](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L202)

Block weight in weight units
