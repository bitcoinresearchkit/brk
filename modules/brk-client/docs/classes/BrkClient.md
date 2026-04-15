[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClient

# Class: BrkClient

Defined in: [Developer/brk/modules/brk-client/index.js:7007](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L7007)

Main BRK client with series tree and API methods

## Extends

- `BrkClientBase`

## Constructors

### Constructor

> **new BrkClient**(`options`): `BrkClient`

Defined in: [Developer/brk/modules/brk-client/index.js:8186](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L8186)

#### Parameters

##### options

`string` \| [`BrkClientOptions`](../interfaces/BrkClientOptions.md)

#### Returns

`BrkClient`

#### Overrides

`BrkClientBase.constructor`

## Properties

### \_cache

> **\_cache**: `Cache` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1594](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1594)

#### Inherited from

`BrkClientBase._cache`

***

### \_cachePromise

> **\_cachePromise**: `Promise`\<`Cache` \| `null`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1592](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1592)

#### Inherited from

`BrkClientBase._cachePromise`

***

### series

> **series**: [`SeriesTree`](../interfaces/SeriesTree.md)

Defined in: [Developer/brk/modules/brk-client/index.js:8189](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L8189)

## Methods

### \_fetchSeriesData()

> **\_fetchSeriesData**\<`T`\>(`path`, `onUpdate?`): `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1697](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1697)

Fetch series data and wrap with helper methods (internal)

#### Type Parameters

##### T

`T`

#### Parameters

##### path

`string`

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

#### Inherited from

`BrkClientBase._fetchSeriesData`

***

### \_getCached()

> **\_getCached**\<`T`\>(`path`, `parse`, `options?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1621](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1621)

Make a GET request - races cache vs network, first to resolve calls onUpdate.
Shared implementation backing `getJson` and `getText`.

#### Type Parameters

##### T

`T`

#### Parameters

##### path

`string`

##### parse

(`res`) => `Promise`\<`T`\>

Response body reader

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`T`\>

#### Inherited from

`BrkClientBase._getCached`

***

### dateToIndex()

> **dateToIndex**(`index`, `d`): `number`

Defined in: [Developer/brk/modules/brk-client/index.js:8178](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L8178)

Convert a Date to an index value for date-based indexes.

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

The index type

##### d

`Date`

The date to convert

#### Returns

`number`

***

### get()

> **get**(`path`, `options?`): `Promise`\<`Response`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1603](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1603)

#### Parameters

##### path

`string`

##### options?

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`Response`\>

#### Inherited from

`BrkClientBase.get`

***

### getAddress()

> **getAddress**(`address`, `options?`): `Promise`\<[`AddrStats`](../interfaces/AddrStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9965](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L9965)

Address information

Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*

Endpoint: `GET /api/address/{address}`

#### Parameters

##### address

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`AddrStats`](../interfaces/AddrStats.md)\>

***

### getAddressConfirmedTxs()

> **getAddressConfirmedTxs**(`address`, `after_txid?`, `options?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10006](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10006)

Address confirmed transactions

Get confirmed transactions for an address, 25 per page. Use ?after_txid=<txid> for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*

Endpoint: `GET /api/address/{address}/txs/chain`

#### Parameters

##### address

`string`

##### after\_txid?

`string`

Txid to paginate from (return transactions before this one)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getAddressMempoolTxs()

> **getAddressMempoolTxs**(`address`, `options?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10027](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10027)

Address mempool transactions

Get unconfirmed transaction IDs for an address from the mempool (up to 50).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*

Endpoint: `GET /api/address/{address}/txs/mempool`

#### Parameters

##### address

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`[]\>

***

### getAddressTxs()

> **getAddressTxs**(`address`, `after_txid?`, `options?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9984](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L9984)

Address transactions

Get transaction history for an address, sorted with newest first. Returns up to 50 mempool transactions plus the first 25 confirmed transactions. Use ?after_txid=<txid> for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*

Endpoint: `GET /api/address/{address}/txs`

#### Parameters

##### address

`string`

##### after\_txid?

`string`

Txid to paginate from (return transactions before this one)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getAddressUtxos()

> **getAddressUtxos**(`address`, `options?`): `Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10045](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10045)

Address UTXOs

Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*

Endpoint: `GET /api/address/{address}/utxo`

#### Parameters

##### address

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

***

### getApi()

> **getApi**(`options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9947](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L9947)

Compact OpenAPI specification

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlock()

> **getBlock**(`hash`, `options?`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10081](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10081)

Block information

Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block)*

Endpoint: `GET /api/block/{hash}`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

***

### getBlockByHeight()

> **getBlockByHeight**(`height`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10063](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10063)

Block hash by height

Retrieve the block hash at a given height. Returns the hash as plain text.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*

Endpoint: `GET /api/block-height/{height}`

#### Parameters

##### height

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlockByTimestamp()

> **getBlockByTimestamp**(`timestamp`, `options?`): `Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11065](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11065)

Block by timestamp

Find the block closest to a given UNIX timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*

Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`

#### Parameters

##### timestamp

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

***

### getBlockFeeRates()

> **getBlockFeeRates**(`time_period`, `options?`): `Promise`\<[`BlockFeeRatesEntry`](../interfaces/BlockFeeRatesEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10993](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10993)

Block fee rates

Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-feerates)*

Endpoint: `GET /api/v1/mining/blocks/fee-rates/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockFeeRatesEntry`](../interfaces/BlockFeeRatesEntry.md)[]\>

***

### getBlockFees()

> **getBlockFees**(`time_period`, `options?`): `Promise`\<[`BlockFeesEntry`](../interfaces/BlockFeesEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11011](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11011)

Block fees

Get average total fees per block for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*

Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockFeesEntry`](../interfaces/BlockFeesEntry.md)[]\>

***

### getBlockHeader()

> **getBlockHeader**(`hash`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10099](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10099)

Block header

Returns the hex-encoded 80-byte block header.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-header)*

Endpoint: `GET /api/block/{hash}/header`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlockRaw()

> **getBlockRaw**(`hash`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10117](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10117)

Raw block

Returns the raw block data in binary format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-raw)*

Endpoint: `GET /api/block/{hash}/raw`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlockRewards()

> **getBlockRewards**(`time_period`, `options?`): `Promise`\<[`BlockRewardsEntry`](../interfaces/BlockRewardsEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11029](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11029)

Block rewards

Get average coinbase reward (subsidy + fees) per block for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*

Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockRewardsEntry`](../interfaces/BlockRewardsEntry.md)[]\>

***

### getBlocks()

> **getBlocks**(`options?`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10225](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10225)

Recent blocks

Retrieve the last 10 blocks. Returns block metadata for each block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlocksFromHeight()

> **getBlocksFromHeight**(`height`, `options?`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10275](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10275)

Blocks from height

Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks/{height}`

#### Parameters

##### height

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlockSizesWeights()

> **getBlockSizesWeights**(`time_period`, `options?`): `Promise`\<[`BlockSizesWeights`](../interfaces/BlockSizesWeights.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11047](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11047)

Block sizes and weights

Get average block sizes and weights for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*

Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockSizesWeights`](../interfaces/BlockSizesWeights.md)\>

***

### getBlockStatus()

> **getBlockStatus**(`hash`, `options?`): `Promise`\<[`BlockStatus`](../interfaces/BlockStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10135](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10135)

Block status

Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-status)*

Endpoint: `GET /api/block/{hash}/status`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockStatus`](../interfaces/BlockStatus.md)\>

***

### getBlocksV1()

> **getBlocksV1**(`options?`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10854](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10854)

Recent blocks with extras

Retrieve the last 10 blocks with extended data including pool identification and fee statistics.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks-v1)*

Endpoint: `GET /api/v1/blocks`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getBlocksV1FromHeight()

> **getBlocksV1FromHeight**(`height`, `options?`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10872](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10872)

Blocks from height with extras

Retrieve up to 10 blocks with extended data going backwards from the given height.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks-v1)*

Endpoint: `GET /api/v1/blocks/{height}`

#### Parameters

##### height

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getBlockTipHash()

> **getBlockTipHash**(`options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10241](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10241)

Block tip hash

Returns the hash of the last block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-tip-hash)*

Endpoint: `GET /api/blocks/tip/hash`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlockTipHeight()

> **getBlockTipHeight**(`options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10257](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10257)

Block tip height

Returns the height of the last block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-tip-height)*

Endpoint: `GET /api/blocks/tip/height`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlockTxid()

> **getBlockTxid**(`hash`, `index`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10154](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10154)

Transaction ID at index

Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-id)*

Endpoint: `GET /api/block/{hash}/txid/{index}`

#### Parameters

##### hash

`string`

Bitcoin block hash

##### index

`number`

Transaction index within the block (0-based)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getBlockTxids()

> **getBlockTxids**(`hash`, `options?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10172](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10172)

Block transaction IDs

Retrieve all transaction IDs in a block. Returns an array of txids in block order.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-ids)*

Endpoint: `GET /api/block/{hash}/txids`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`[]\>

***

### getBlockTxs()

> **getBlockTxs**(`hash`, `options?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10190](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10190)

Block transactions

Retrieve transactions in a block by block hash. Returns up to 25 transactions starting from index 0.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*

Endpoint: `GET /api/block/{hash}/txs`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getBlockTxsFromIndex()

> **getBlockTxsFromIndex**(`hash`, `start_index`, `options?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10209](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10209)

Block transactions (paginated)

Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*

Endpoint: `GET /api/block/{hash}/txs/{start_index}`

#### Parameters

##### hash

`string`

Bitcoin block hash

##### start\_index

`number`

Starting transaction index within the block (0-based)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getBlockV1()

> **getBlockV1**(`hash`, `options?`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10838](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10838)

Block (v1)

Returns block details with extras by hash.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-v1)*

Endpoint: `GET /api/v1/block/{hash}`

#### Parameters

##### hash

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)\>

***

### getCostBasis()

> **getCostBasis**(`cohort`, `date`, `bucket?`, `value?`, `options?`): `Promise`\<`Object`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10434](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10434)

Cost basis distribution

Get the cost basis distribution for a cohort on a specific date.

Query params:
- `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100
- `value`: supply (default, in BTC), realized (USD), unrealized (USD)

Endpoint: `GET /api/series/cost-basis/{cohort}/{date}`

#### Parameters

##### cohort

`string`

##### date

`string`

##### bucket?

[`CostBasisBucket`](../type-aliases/CostBasisBucket.md)

Bucket type for aggregation. Default: raw (no aggregation).

##### value?

[`CostBasisValue`](../type-aliases/CostBasisValue.md)

Value type to return. Default: supply.

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`Object`\>

***

### getCostBasisCohorts()

> **getCostBasisCohorts**(`options?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10395](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10395)

Available cost basis cohorts

List available cohorts for cost basis distribution.

Endpoint: `GET /api/series/cost-basis`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`[]\>

***

### getCostBasisDates()

> **getCostBasisDates**(`cohort`, `options?`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10411](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10411)

Available cost basis dates

List available dates for a cohort's cost basis distribution.

Endpoint: `GET /api/series/cost-basis/{cohort}/dates`

#### Parameters

##### cohort

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`number`[]\>

***

### getCpfp()

> **getCpfp**(`txid`, `options?`): `Promise`\<[`CpfpInfo`](../interfaces/CpfpInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10890](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10890)

CPFP info

Returns ancestors and descendants for a CPFP (Child Pays For Parent) transaction, including the effective fee rate of the package.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-children-pay-for-parent)*

Endpoint: `GET /api/v1/cpfp/{txid}`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`CpfpInfo`](../interfaces/CpfpInfo.md)\>

***

### getDifficultyAdjustment()

> **getDifficultyAdjustment**(`options?`): `Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10906](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10906)

Difficulty adjustment

Get current difficulty adjustment progress and estimates.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

***

### getDifficultyAdjustments()

> **getDifficultyAdjustments**(`options?`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11081](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11081)

Difficulty adjustments (all time)

Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDifficultyAdjustmentsByPeriod()

> **getDifficultyAdjustmentsByPeriod**(`time_period`, `options?`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11099](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11099)

Difficulty adjustments

Get historical difficulty adjustments for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDiskUsage()

> **getDiskUsage**(`options?`): `Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10645](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10645)

Disk usage

Returns the disk space used by BRK and Bitcoin data.

Endpoint: `GET /api/server/disk`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

***

### getHashrate()

> **getHashrate**(`options?`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11115](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11115)

Network hashrate (all time)

Get network hashrate and difficulty data for all time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHashrateByPeriod()

> **getHashrateByPeriod**(`time_period`, `options?`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11167](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11167)

Network hashrate

Get network hashrate and difficulty data for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHealth()

> **getHealth**(`options?`): `Promise`\<[`Health`](../interfaces/Health.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11356](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11356)

Health check

Returns the health status of the API server, including uptime information.

Endpoint: `GET /health`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Health`](../interfaces/Health.md)\>

***

### getHistoricalPrice()

> **getHistoricalPrice**(`timestamp?`, `options?`): `Promise`\<[`HistoricalPrice`](../interfaces/HistoricalPrice.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10972](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10972)

Historical price

Get historical BTC/USD price. Optionally specify a UNIX timestamp to get the price at that time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-historical-price)*

Endpoint: `GET /api/v1/historical-price`

#### Parameters

##### timestamp?

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`HistoricalPrice`](../interfaces/HistoricalPrice.md)\>

***

### getIndexes()

> **getIndexes**(`options?`): `Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10466](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10466)

List available indexes

Returns all available indexes with their accepted query aliases. Use any alias when querying series.

Endpoint: `GET /api/series/indexes`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

***

### getJson()

> **getJson**\<`T`\>(`path`, `options?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1675](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1675)

Make a GET request expecting a JSON response. Cached and supports `onUpdate`.

#### Type Parameters

##### T

`T`

#### Parameters

##### path

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`T`\>

#### Inherited from

`BrkClientBase.getJson`

***

### getLivePrice()

> **getLivePrice**(`options?`): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10305](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10305)

Live BTC/USD price

Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.

Endpoint: `GET /api/mempool/price`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`number`\>

***

### getMempool()

> **getMempool**(`options?`): `Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10291](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10291)

Mempool statistics

Get current mempool statistics including transaction count, total vsize, total fees, and fee histogram.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

***

### getMempoolBlocks()

> **getMempoolBlocks**(`options?`): `Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10922](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10922)

Projected mempool blocks

Get projected blocks from the mempool for fee estimation.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

***

### getMempoolRecent()

> **getMempoolRecent**(`options?`): `Promise`\<[`MempoolRecentTx`](../interfaces/MempoolRecentTx.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10321](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10321)

Recent mempool transactions

Get the last 10 transactions to enter the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-recent)*

Endpoint: `GET /api/mempool/recent`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`MempoolRecentTx`](../interfaces/MempoolRecentTx.md)[]\>

***

### getMempoolTxids()

> **getMempoolTxids**(`options?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10337](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10337)

Mempool transaction IDs

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`[]\>

***

### getOpenapi()

> **getOpenapi**(`options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:11370](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11370)

OpenAPI specification

Full OpenAPI 3.1 specification for this API.

Endpoint: `GET /openapi.json`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getPool()

> **getPool**(`slug`, `options?`): `Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11185](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11185)

Mining pool details

Get detailed information about a specific mining pool including block counts and shares for different time periods.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool)*

Endpoint: `GET /api/v1/mining/pool/{slug}`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

***

### getPoolBlocks()

> **getPoolBlocks**(`slug`, `options?`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11203](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11203)

Mining pool blocks

Get the 10 most recent blocks mined by a specific pool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-blocks)*

Endpoint: `GET /api/v1/mining/pool/{slug}/blocks`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getPoolBlocksFrom()

> **getPoolBlocksFrom**(`slug`, `height`, `options?`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11222](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11222)

Mining pool blocks from height

Get 10 blocks mined by a specific pool before (and including) the given height.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-blocks)*

Endpoint: `GET /api/v1/mining/pool/{slug}/blocks/{height}`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

##### height

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getPoolHashrate()

> **getPoolHashrate**(`slug`, `options?`): `Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11240](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11240)

Mining pool hashrate

Get hashrate history for a specific mining pool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrate)*

Endpoint: `GET /api/v1/mining/pool/{slug}/hashrate`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

***

### getPools()

> **getPools**(`options?`): `Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11256](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11256)

List all mining pools

Get list of all known mining pools with their identifiers.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

***

### getPoolsHashrate()

> **getPoolsHashrate**(`options?`): `Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11131](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11131)

All pools hashrate (all time)

Get hashrate data for all mining pools.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrates)*

Endpoint: `GET /api/v1/mining/hashrate/pools`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

***

### getPoolsHashrateByPeriod()

> **getPoolsHashrateByPeriod**(`time_period`, `options?`): `Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11149](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11149)

All pools hashrate

Get hashrate data for all mining pools for a time period. Valid periods: `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrates)*

Endpoint: `GET /api/v1/mining/hashrate/pools/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

***

### getPoolStats()

> **getPoolStats**(`time_period`, `options?`): `Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11274](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11274)

Mining pool statistics

Get mining pool statistics for a time period. Valid periods: `24h`, `3d`, `1w`, `1m`, `3m`, `6m`, `1y`, `2y`, `3y`.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

***

### getPreciseFees()

> **getPreciseFees**(`options?`): `Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10938](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10938)

Precise recommended fees

Get recommended fee rates with up to 3 decimal places.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees-precise)*

Endpoint: `GET /api/v1/fees/precise`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getPrices()

> **getPrices**(`options?`): `Promise`\<[`Prices`](../interfaces/Prices.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11308](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11308)

Current BTC price

Returns bitcoin latest price (on-chain derived, USD only).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-price)*

Endpoint: `GET /api/v1/prices`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Prices`](../interfaces/Prices.md)\>

***

### getRecommendedFees()

> **getRecommendedFees**(`options?`): `Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10954](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10954)

Recommended fees

Get recommended fee rates for different confirmation targets.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getRewardStats()

> **getRewardStats**(`block_count`, `options?`): `Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11292](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11292)

Mining reward statistics

Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*

Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`

#### Parameters

##### block\_count

`number`

Number of recent blocks to include

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

***

### getSeries()

> **getSeries**(`series`, `index`, `start?`, `end?`, `limit?`, `format?`, `options?`): `Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10545](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10545)

Get series data

Fetch data for a specific series at the given index. Use query parameters to filter by date range and format (json/csv).

Endpoint: `GET /api/series/{series}/{index}`

#### Parameters

##### series

`string`

Series name

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### start?

`number`

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`

##### end?

`number`

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

##### limit?

`number`

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)\>

***

### getSeriesBulk()

> **getSeriesBulk**(`series?`, `index?`, `start?`, `end?`, `limit?`, `format?`, `options?`): `Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10372](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10372)

Bulk series data

Fetch multiple series in a single request. Supports filtering by index and date range. Returns an array of SeriesData objects. For a single series, use `get_series` instead.

Endpoint: `GET /api/series/bulk`

#### Parameters

##### series?

`string`

Requested series

##### index?

[`Index`](../type-aliases/Index.md)

Index to query

##### start?

`number`

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`

##### end?

`number`

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

##### limit?

`number`

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)[]\>

***

### getSeriesCount()

> **getSeriesCount**(`options?`): `Promise`\<[`SeriesCount`](../interfaces/SeriesCount.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10452](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10452)

Series count

Returns the number of series available per index type.

Endpoint: `GET /api/series/count`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`SeriesCount`](../interfaces/SeriesCount.md)[]\>

***

### getSeriesData()

> **getSeriesData**(`series`, `index`, `start?`, `end?`, `limit?`, `format?`, `options?`): `Promise`\<`string` \| `boolean`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10573](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10573)

Get raw series data

Returns just the data array without the SeriesData wrapper. Supports the same range and format parameters as the standard endpoint.

Endpoint: `GET /api/series/{series}/{index}/data`

#### Parameters

##### series

`string`

Series name

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### start?

`number`

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`

##### end?

`number`

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

##### limit?

`number`

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string` \| `boolean`[]\>

***

### getSeriesInfo()

> **getSeriesInfo**(`series`, `options?`): `Promise`\<[`SeriesInfo`](../interfaces/SeriesInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10524](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10524)

Get series info

Returns the supported indexes and value type for the specified series.

Endpoint: `GET /api/series/{series}`

#### Parameters

##### series

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`SeriesInfo`](../interfaces/SeriesInfo.md)\>

***

### getSeriesLatest()

> **getSeriesLatest**(`series`, `index`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10597](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10597)

Get latest series value

Returns the single most recent value for a series, unwrapped (not inside a SeriesData object).

Endpoint: `GET /api/series/{series}/{index}/latest`

#### Parameters

##### series

`string`

Series name

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getSeriesLen()

> **getSeriesLen**(`series`, `index`, `options?`): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10614](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10614)

Get series data length

Returns the total number of data points for a series at the given index.

Endpoint: `GET /api/series/{series}/{index}/len`

#### Parameters

##### series

`string`

Series name

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`number`\>

***

### getSeriesTree()

> **getSeriesTree**(`options?`): `Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10351](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10351)

Series catalog

Returns the complete hierarchical catalog of available series organized as a tree structure. Series are grouped by categories and subcategories.

Endpoint: `GET /api/series`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

***

### getSeriesVersion()

> **getSeriesVersion**(`series`, `index`, `options?`): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10631](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10631)

Get series version

Returns the current version of a series. Changes when the series data is updated.

Endpoint: `GET /api/series/{series}/{index}/version`

#### Parameters

##### series

`string`

Series name

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`number`\>

***

### getSyncStatus()

> **getSyncStatus**(`options?`): `Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10659](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10659)

Sync status

Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

Endpoint: `GET /api/server/sync`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

***

### getText()

> **getText**(`path`, `options?`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1686](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L1686)

Make a GET request expecting a text response (text/plain, text/csv, ...).
Cached and supports `onUpdate`, same as `getJson`.

#### Parameters

##### path

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`\>

#### Inherited from

`BrkClientBase.getText`

***

### getTransactionTimes()

> **getTransactionTimes**(`options?`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:11324](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11324)

Transaction first-seen times

Returns timestamps when transactions were first seen in the mempool. Returns 0 for mined or unknown transactions.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-times)*

Endpoint: `GET /api/v1/transaction-times`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`number`[]\>

***

### getTx()

> **getTx**(`txid`, `options?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10693](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10693)

Transaction information

Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*

Endpoint: `GET /api/tx/{txid}`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

***

### getTxByIndex()

> **getTxByIndex**(`index`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10675](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10675)

Txid by index

Retrieve the transaction ID (txid) at a given global transaction index. Returns the txid as plain text.

Endpoint: `GET /api/tx-index/{index}`

#### Parameters

##### index

`number`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getTxHex()

> **getTxHex**(`txid`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10711](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10711)

Transaction hex

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*

Endpoint: `GET /api/tx/{txid}/hex`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getTxMerkleblockProof()

> **getTxMerkleblockProof**(`txid`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10747](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10747)

Transaction merkleblock proof

Get the merkleblock proof for a transaction (BIP37 format, hex encoded).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkleblock-proof)*

Endpoint: `GET /api/tx/{txid}/merkleblock-proof`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getTxMerkleProof()

> **getTxMerkleProof**(`txid`, `options?`): `Promise`\<[`MerkleProof`](../interfaces/MerkleProof.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10729](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10729)

Transaction merkle proof

Get the merkle inclusion proof for a transaction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkle-proof)*

Endpoint: `GET /api/tx/{txid}/merkle-proof`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`MerkleProof`](../interfaces/MerkleProof.md)\>

***

### getTxOutspend()

> **getTxOutspend**(`txid`, `vout`, `options?`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10766](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10766)

Output spend status

Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*

Endpoint: `GET /api/tx/{txid}/outspend/{vout}`

#### Parameters

##### txid

`string`

Transaction ID

##### vout

`number`

Output index

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)\>

***

### getTxOutspends()

> **getTxOutspends**(`txid`, `options?`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10784](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10784)

All output spend statuses

Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*

Endpoint: `GET /api/tx/{txid}/outspends`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)[]\>

***

### getTxRaw()

> **getTxRaw**(`txid`, `options?`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10802](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10802)

Transaction raw

Returns a transaction as binary data.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-raw)*

Endpoint: `GET /api/tx/{txid}/raw`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`any`\>

***

### getTxStatus()

> **getTxStatus**(`txid`, `options?`): `Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10820](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10820)

Transaction status

Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*

Endpoint: `GET /api/tx/{txid}/status`

#### Parameters

##### txid

`string`

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

***

### getVersion()

> **getVersion**(`options?`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:11384](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11384)

API version

Returns the current version of the API server

Endpoint: `GET /version`

#### Parameters

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`\>

***

### indexToDate()

> **indexToDate**(`index`, `i`): `Date`

Defined in: [Developer/brk/modules/brk-client/index.js:8168](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L8168)

Convert an index value to a Date for date-based indexes.

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

The index type

##### i

`number`

The index value

#### Returns

`Date`

***

### listSeries()

> **listSeries**(`page?`, `per_page?`, `options?`): `Promise`\<[`PaginatedSeries`](../interfaces/PaginatedSeries.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10483](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10483)

Series list

Paginated flat list of all available series names. Use `page` query param for pagination.

Endpoint: `GET /api/series/list`

#### Parameters

##### page?

`number`

Pagination index

##### per\_page?

`number`

Results per page (default: 1000, max: 1000)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`PaginatedSeries`](../interfaces/PaginatedSeries.md)\>

***

### searchSeries()

> **searchSeries**(`q?`, `limit?`, `options?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10504](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L10504)

Search series

Fuzzy search for series by name. Supports partial matches and typos.

Endpoint: `GET /api/series/search`

#### Parameters

##### q?

`string`

Search query string

##### limit?

`number`

Maximum number of results

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<`string`[]\>

***

### seriesEndpoint()

> **seriesEndpoint**(`series`, `index`): [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`unknown`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9934](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L9934)

Create a dynamic series endpoint builder for any series/index combination.

Use this for programmatic access when the series name is determined at runtime.
For type-safe access, use the `series` tree instead.

#### Parameters

##### series

`string`

The series name

##### index

[`Index`](../type-aliases/Index.md)

The index name

#### Returns

[`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`unknown`\>

***

### validateAddress()

> **validateAddress**(`address`, `options?`): `Promise`\<[`AddrValidation`](../interfaces/AddrValidation.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:11342](https://github.com/bitcoinresearchkit/brk/blob/75a97b4da99e60fce9ac789d118004f9b3db3ee5/modules/brk-client/index.js#L11342)

Validate address

Validate a Bitcoin address and get information about its type and scriptPubKey. Returns `isvalid: false` with an error message for invalid addresses.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*

Endpoint: `GET /api/v1/validate-address/{address}`

#### Parameters

##### address

`string`

Bitcoin address to validate (can be any string)

##### options?

###### onUpdate?

(`value`) => `void`

###### signal?

`AbortSignal`

#### Returns

`Promise`\<[`AddrValidation`](../interfaces/AddrValidation.md)\>
