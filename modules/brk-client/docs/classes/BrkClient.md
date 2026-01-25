[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClient

# Class: BrkClient

Defined in: [Developer/brk/modules/brk-client/index.js:4283](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L4283)

Main BRK client with metrics tree and API methods

## Extends

- `BrkClientBase`

## Constructors

### Constructor

> **new BrkClient**(`options`): `BrkClient`

Defined in: [Developer/brk/modules/brk-client/index.js:5181](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L5181)

#### Parameters

##### options

`string` | [`BrkClientOptions`](../interfaces/BrkClientOptions.md)

#### Returns

`BrkClient`

#### Overrides

`BrkClientBase.constructor`

## Properties

### \_cachePromise

> **\_cachePromise**: `Promise`\<`Cache` \| `null`\>

Defined in: [Developer/brk/modules/brk-client/index.js:995](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L995)

#### Inherited from

`BrkClientBase._cachePromise`

***

### metrics

> **metrics**: [`MetricsTree`](../interfaces/MetricsTree.md)

Defined in: [Developer/brk/modules/brk-client/index.js:5184](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L5184)

## Methods

### get()

> **get**(`path`): `Promise`\<`Response`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1002](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L1002)

#### Parameters

##### path

`string`

#### Returns

`Promise`\<`Response`\>

#### Inherited from

`BrkClientBase.get`

***

### getAddress()

> **getAddress**(`address`): `Promise`\<[`AddressStats`](../interfaces/AddressStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6274](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6274)

Address information

Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*

Endpoint: `GET /api/address/{address}`

#### Parameters

##### address

`string`

#### Returns

`Promise`\<[`AddressStats`](../interfaces/AddressStats.md)\>

***

### getAddressConfirmedTxs()

> **getAddressConfirmedTxs**(`address`, `after_txid?`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6315](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6315)

Address confirmed transactions

Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*

Endpoint: `GET /api/address/{address}/txs/chain`

#### Parameters

##### address

`string`

##### after\_txid?

`string`

Txid to paginate from (return transactions before this one)

##### limit?

`number`

Maximum number of results to return. Defaults to 25 if not specified.

#### Returns

`Promise`\<`string`[]\>

***

### getAddressMempoolTxs()

> **getAddressMempoolTxs**(`address`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6336](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6336)

Address mempool transactions

Get unconfirmed transaction IDs for an address from the mempool (up to 50).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*

Endpoint: `GET /api/address/{address}/txs/mempool`

#### Parameters

##### address

`string`

#### Returns

`Promise`\<`string`[]\>

***

### getAddressTxs()

> **getAddressTxs**(`address`, `after_txid?`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6292](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6292)

Address transaction IDs

Get transaction IDs for an address, newest first. Use after_txid for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*

Endpoint: `GET /api/address/{address}/txs`

#### Parameters

##### address

`string`

##### after\_txid?

`string`

Txid to paginate from (return transactions before this one)

##### limit?

`number`

Maximum number of results to return. Defaults to 25 if not specified.

#### Returns

`Promise`\<`string`[]\>

***

### getAddressUtxos()

> **getAddressUtxos**(`address`): `Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6352](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6352)

Address UTXOs

Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*

Endpoint: `GET /api/address/{address}/utxo`

#### Parameters

##### address

`string`

#### Returns

`Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

***

### getApi()

> **getApi**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:6258](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6258)

Compact OpenAPI specification

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

#### Returns

`Promise`\<`any`\>

***

### getBlock()

> **getBlock**(`hash`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6384](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6384)

Block information

Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block)*

Endpoint: `GET /api/block/{hash}`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

***

### getBlockByHeight()

> **getBlockByHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6368](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6368)

Block by height

Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*

Endpoint: `GET /api/block-height/{height}`

#### Parameters

##### height

`number`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

***

### getBlockByTimestamp()

> **getBlockByTimestamp**(`timestamp`): `Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6898](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6898)

Block by timestamp

Find the block closest to a given UNIX timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*

Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`

#### Parameters

##### timestamp

`number`

#### Returns

`Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

***

### getBlockFeeRates()

> **getBlockFeeRates**(`time_period`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:6834](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6834)

Block fee rates (WIP)

**Work in progress.** Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-feerates)*

Endpoint: `GET /api/v1/mining/blocks/fee-rates/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<`any`\>

***

### getBlockFees()

> **getBlockFees**(`time_period`): `Promise`\<[`BlockFeesEntry`](../interfaces/BlockFeesEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6850](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6850)

Block fees

Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*

Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`BlockFeesEntry`](../interfaces/BlockFeesEntry.md)[]\>

***

### getBlockRaw()

> **getBlockRaw**(`hash`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6400](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6400)

Raw block

Returns the raw block data in binary format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-raw)*

Endpoint: `GET /api/block/{hash}/raw`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<`number`[]\>

***

### getBlockRewards()

> **getBlockRewards**(`time_period`): `Promise`\<[`BlockRewardsEntry`](../interfaces/BlockRewardsEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6866](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6866)

Block rewards

Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*

Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`BlockRewardsEntry`](../interfaces/BlockRewardsEntry.md)[]\>

***

### getBlocks()

> **getBlocks**(): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6480](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6480)

Recent blocks

Retrieve the last 10 blocks. Returns block metadata for each block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlocksFromHeight()

> **getBlocksFromHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6496](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6496)

Blocks from height

Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks/{height}`

#### Parameters

##### height

`number`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlockSizesWeights()

> **getBlockSizesWeights**(`time_period`): `Promise`\<[`BlockSizesWeights`](../interfaces/BlockSizesWeights.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6882](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6882)

Block sizes and weights

Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*

Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`BlockSizesWeights`](../interfaces/BlockSizesWeights.md)\>

***

### getBlockStatus()

> **getBlockStatus**(`hash`): `Promise`\<[`BlockStatus`](../interfaces/BlockStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6416](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6416)

Block status

Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-status)*

Endpoint: `GET /api/block/{hash}/status`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<[`BlockStatus`](../interfaces/BlockStatus.md)\>

***

### getBlockTxid()

> **getBlockTxid**(`hash`, `index`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:6433](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6433)

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

#### Returns

`Promise`\<`string`\>

***

### getBlockTxids()

> **getBlockTxids**(`hash`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6449](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6449)

Block transaction IDs

Retrieve all transaction IDs in a block. Returns an array of txids in block order.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-ids)*

Endpoint: `GET /api/block/{hash}/txids`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<`string`[]\>

***

### getBlockTxs()

> **getBlockTxs**(`hash`, `start_index`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6466](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6466)

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

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getDifficultyAdjustment()

> **getDifficultyAdjustment**(): `Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6790](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6790)

Difficulty adjustment

Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

#### Returns

`Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

***

### getDifficultyAdjustments()

> **getDifficultyAdjustments**(): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6912](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6912)

Difficulty adjustments (all time)

Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments`

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDifficultyAdjustmentsByPeriod()

> **getDifficultyAdjustmentsByPeriod**(`time_period`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6928](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6928)

Difficulty adjustments

Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDiskUsage()

> **getDiskUsage**(): `Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6683](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6683)

Disk usage

Returns the disk space used by BRK and Bitcoin data.

Endpoint: `GET /api/server/disk`

#### Returns

`Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

***

### getHashrate()

> **getHashrate**(): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6942](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6942)

Network hashrate (all time)

Get network hashrate and difficulty data for all time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate`

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHashrateByPeriod()

> **getHashrateByPeriod**(`time_period`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6958](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6958)

Network hashrate

Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHealth()

> **getHealth**(): `Promise`\<[`Health`](../interfaces/Health.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7048](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L7048)

Health check

Returns the health status of the API server, including uptime information.

Endpoint: `GET /health`

#### Returns

`Promise`\<[`Health`](../interfaces/Health.md)\>

***

### getIndexes()

> **getIndexes**(): `Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6634](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6634)

List available indexes

Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.

Endpoint: `GET /api/metrics/indexes`

#### Returns

`Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

***

### getJson()

> **getJson**\<`T`\>(`path`, `onUpdate?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1017](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L1017)

Make a GET request with stale-while-revalidate caching

#### Type Parameters

##### T

`T`

#### Parameters

##### path

`string`

##### onUpdate?

(`value`) => `void`

Called when data is available

#### Returns

`Promise`\<`T`\>

#### Inherited from

`BrkClientBase.getJson`

***

### getMempool()

> **getMempool**(): `Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6510](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6510)

Mempool statistics

Get current mempool statistics including transaction count, total vsize, and total fees.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool/info`

#### Returns

`Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

***

### getMempoolBlocks()

> **getMempoolBlocks**(): `Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6804](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6804)

Projected mempool blocks

Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

#### Returns

`Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

***

### getMempoolTxids()

> **getMempoolTxids**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6524](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6524)

Mempool transaction IDs

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

#### Returns

`Promise`\<`string`[]\>

***

### getMetric()

> **getMetric**(`metric`, `index`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| [`AnyMetricData`](../type-aliases/AnyMetricData.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6557](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6557)

Get metric data

Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).

Endpoint: `GET /api/metric/{metric}/{index}`

#### Parameters

##### metric

`string`

Metric name

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### start?

`number`

Inclusive starting index, if negative counts from end

##### end?

`number`

Exclusive ending index, if negative counts from end

##### limit?

`string`

Maximum number of values to return (ignored if `end` is set)

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

#### Returns

`Promise`\<`string` \| [`AnyMetricData`](../type-aliases/AnyMetricData.md)\>

***

### getMetricInfo()

> **getMetricInfo**(`metric`): `Promise`\<[`Index`](../type-aliases/Index.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6538](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6538)

Get supported indexes for a metric

Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.

Endpoint: `GET /api/metric/{metric}`

#### Parameters

##### metric

`string`

#### Returns

`Promise`\<[`Index`](../type-aliases/Index.md)[]\>

***

### getMetrics()

> **getMetrics**(`metrics?`, `index?`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| [`AnyMetricData`](../type-aliases/AnyMetricData.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6598](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6598)

Bulk metric data

Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects. For a single metric, use `get_metric` instead.

Endpoint: `GET /api/metrics/bulk`

#### Parameters

##### metrics?

`string`

Requested metrics

##### index?

[`Index`](../type-aliases/Index.md)

Index to query

##### start?

`number`

Inclusive starting index, if negative counts from end

##### end?

`number`

Exclusive ending index, if negative counts from end

##### limit?

`string`

Maximum number of values to return (ignored if `end` is set)

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

#### Returns

`Promise`\<`string` \| [`AnyMetricData`](../type-aliases/AnyMetricData.md)[]\>

***

### getMetricsCount()

> **getMetricsCount**(): `Promise`\<[`MetricCount`](../interfaces/MetricCount.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6622](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6622)

Metric count

Returns the number of metrics available per index type.

Endpoint: `GET /api/metrics/count`

#### Returns

`Promise`\<[`MetricCount`](../interfaces/MetricCount.md)[]\>

***

### getMetricsTree()

> **getMetricsTree**(): `Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6579](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6579)

Metrics catalog

Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.

Endpoint: `GET /api/metrics`

#### Returns

`Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

***

### getOpenapi()

> **getOpenapi**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7060](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L7060)

OpenAPI specification

Full OpenAPI 3.1 specification for this API.

Endpoint: `GET /openapi.json`

#### Returns

`Promise`\<`any`\>

***

### getPool()

> **getPool**(`slug`): `Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6974](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6974)

Mining pool details

Get detailed information about a specific mining pool including block counts and shares for different time periods.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool)*

Endpoint: `GET /api/v1/mining/pool/{slug}`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

#### Returns

`Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

***

### getPools()

> **getPools**(): `Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6988](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6988)

List all mining pools

Get list of all known mining pools with their identifiers.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools`

#### Returns

`Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

***

### getPoolStats()

> **getPoolStats**(`time_period`): `Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7004](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L7004)

Mining pool statistics

Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

***

### getRecommendedFees()

> **getRecommendedFees**(): `Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6818](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6818)

Recommended fees

Get recommended fee rates for different confirmation targets based on current mempool state.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getRewardStats()

> **getRewardStats**(`block_count`): `Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7020](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L7020)

Mining reward statistics

Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*

Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`

#### Parameters

##### block\_count

`number`

Number of recent blocks to include

#### Returns

`Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

***

### getSyncStatus()

> **getSyncStatus**(): `Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6695](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6695)

Sync status

Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

Endpoint: `GET /api/server/sync`

#### Returns

`Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

***

### getText()

> **getText**(`path`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1050](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L1050)

Make a GET request and return raw text (for CSV responses)

#### Parameters

##### path

`string`

#### Returns

`Promise`\<`string`\>

#### Inherited from

`BrkClientBase.getText`

***

### getTx()

> **getTx**(`txid`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6711](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6711)

Transaction information

Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*

Endpoint: `GET /api/tx/{txid}`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

***

### getTxHex()

> **getTxHex**(`txid`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:6727](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6727)

Transaction hex

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*

Endpoint: `GET /api/tx/{txid}/hex`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<`string`\>

***

### getTxOutspend()

> **getTxOutspend**(`txid`, `vout`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6744](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6744)

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

#### Returns

`Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)\>

***

### getTxOutspends()

> **getTxOutspends**(`txid`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6760](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6760)

All output spend statuses

Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*

Endpoint: `GET /api/tx/{txid}/outspends`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)[]\>

***

### getTxStatus()

> **getTxStatus**(`txid`): `Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6776](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6776)

Transaction status

Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*

Endpoint: `GET /api/tx/{txid}/status`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

***

### getVersion()

> **getVersion**(): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7072](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L7072)

API version

Returns the current version of the API server

Endpoint: `GET /version`

#### Returns

`Promise`\<`string`\>

***

### listMetrics()

> **listMetrics**(`page?`): `Promise`\<[`PaginatedMetrics`](../interfaces/PaginatedMetrics.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:6648](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6648)

Metrics list

Paginated flat list of all available metric names. Use `page` query param for pagination.

Endpoint: `GET /api/metrics/list`

#### Parameters

##### page?

`number`

Pagination index

#### Returns

`Promise`\<[`PaginatedMetrics`](../interfaces/PaginatedMetrics.md)\>

***

### metric()

> **metric**(`metric`, `index`): [`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`unknown`\>

Defined in: [Developer/brk/modules/brk-client/index.js:6246](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6246)

Create a dynamic metric endpoint builder for any metric/index combination.

Use this for programmatic access when the metric name is determined at runtime.
For type-safe access, use the `metrics` tree instead.

#### Parameters

##### metric

`string`

The metric name

##### index

[`Index`](../type-aliases/Index.md)

The index name

#### Returns

[`MetricEndpointBuilder`](../interfaces/MetricEndpointBuilder.md)\<`unknown`\>

***

### searchMetrics()

> **searchMetrics**(`metric`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:6667](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L6667)

Search metrics

Fuzzy search for metrics by name. Supports partial matches and typos.

Endpoint: `GET /api/metrics/search/{metric}`

#### Parameters

##### metric

`string`

##### limit?

`number`

#### Returns

`Promise`\<`string`[]\>

***

### validateAddress()

> **validateAddress**(`address`): `Promise`\<[`AddressValidation`](../interfaces/AddressValidation.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7036](https://github.com/bitcoinresearchkit/brk/blob/36b56a400c96653daf373f383aeefb915799495a/modules/brk-client/index.js#L7036)

Validate address

Validate a Bitcoin address and get information about its type and scriptPubKey.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*

Endpoint: `GET /api/v1/validate-address/{address}`

#### Parameters

##### address

`string`

Bitcoin address to validate (can be any string)

#### Returns

`Promise`\<[`AddressValidation`](../interfaces/AddressValidation.md)\>
