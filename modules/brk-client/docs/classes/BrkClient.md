[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClient

# Class: BrkClient

Defined in: [Developer/brk/modules/brk-client/index.js:5007](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L5007)

Main BRK client with metrics tree and API methods

## Extends

- `BrkClientBase`

## Constructors

### Constructor

> **new BrkClient**(`options`): `BrkClient`

Defined in: [Developer/brk/modules/brk-client/index.js:5924](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L5924)

#### Parameters

##### options

`string` | [`BrkClientOptions`](../interfaces/BrkClientOptions.md)

#### Returns

`BrkClient`

#### Overrides

`BrkClientBase.constructor`

## Properties

### \_cache

> **\_cache**: `Cache` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1167](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1167)

#### Inherited from

`BrkClientBase._cache`

***

### \_cachePromise

> **\_cachePromise**: `Promise`\<`Cache` \| `null`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1165](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1165)

#### Inherited from

`BrkClientBase._cachePromise`

***

### metrics

> **metrics**: [`MetricsTree`](../interfaces/MetricsTree.md)

Defined in: [Developer/brk/modules/brk-client/index.js:5927](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L5927)

## Methods

### \_fetchMetricData()

> **\_fetchMetricData**\<`T`\>(`path`, `onUpdate?`): `Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1257](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1257)

Fetch metric data and wrap with helper methods (internal)

#### Type Parameters

##### T

`T`

#### Parameters

##### path

`string`

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](../interfaces/MetricData.md)\<`T`\>\>

#### Inherited from

`BrkClientBase._fetchMetricData`

***

### get()

> **get**(`path`): `Promise`\<`Response`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1175](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1175)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7125](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7125)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7166](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7166)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7187](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7187)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7143](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7143)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7203](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7203)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7109](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7109)

Compact OpenAPI specification

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

#### Returns

`Promise`\<`any`\>

***

### getBlock()

> **getBlock**(`hash`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7235](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7235)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7219](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7219)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7813](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7813)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7749](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7749)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7765](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7765)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7251](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7251)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7781](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7781)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7331](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7331)

Recent blocks

Retrieve the last 10 blocks. Returns block metadata for each block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlocksFromHeight()

> **getBlocksFromHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7347](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7347)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7797](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7797)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7267](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7267)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7284](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7284)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7300](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7300)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7317](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7317)

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

### getCostBasis()

> **getCostBasis**(`cohort`, `date`, `bucket?`, `value?`): `Promise`\<`Object`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7520](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7520)

Cost basis distribution

Get the cost basis distribution for a cohort on a specific date.

Query params:
- `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100
- `value`: supply (default, in BTC), realized (USD), unrealized (USD)

Endpoint: `GET /api/metrics/cost-basis/{cohort}/{date}`

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

#### Returns

`Promise`\<`Object`\>

***

### getCostBasisCohorts()

> **getCostBasisCohorts**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7485](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7485)

Available cost basis cohorts

List available cohorts for cost basis distribution.

Endpoint: `GET /api/metrics/cost-basis`

#### Returns

`Promise`\<`string`[]\>

***

### getCostBasisDates()

> **getCostBasisDates**(`cohort`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7499](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7499)

Available cost basis dates

List available dates for a cohort's cost basis distribution.

Endpoint: `GET /api/metrics/cost-basis/{cohort}/dates`

#### Parameters

##### cohort

`string`

#### Returns

`Promise`\<`number`[]\>

***

### getDifficultyAdjustment()

> **getDifficultyAdjustment**(): `Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7705](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7705)

Difficulty adjustment

Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

#### Returns

`Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

***

### getDifficultyAdjustments()

> **getDifficultyAdjustments**(): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7827](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7827)

Difficulty adjustments (all time)

Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments`

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDifficultyAdjustmentsByPeriod()

> **getDifficultyAdjustmentsByPeriod**(`time_period`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7843](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7843)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7598](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7598)

Disk usage

Returns the disk space used by BRK and Bitcoin data.

Endpoint: `GET /api/server/disk`

#### Returns

`Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

***

### getHashrate()

> **getHashrate**(): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7857](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7857)

Network hashrate (all time)

Get network hashrate and difficulty data for all time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate`

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHashrateByPeriod()

> **getHashrateByPeriod**(`time_period`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7873](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7873)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7963](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7963)

Health check

Returns the health status of the API server, including uptime information.

Endpoint: `GET /health`

#### Returns

`Promise`\<[`Health`](../interfaces/Health.md)\>

***

### getIndexes()

> **getIndexes**(): `Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7549](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7549)

List available indexes

Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.

Endpoint: `GET /api/metrics/indexes`

#### Returns

`Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

***

### getJson()

> **getJson**\<`T`\>(`path`, `onUpdate?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1190](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1190)

Make a GET request - races cache vs network, first to resolve calls onUpdate

#### Type Parameters

##### T

`T`

#### Parameters

##### path

`string`

##### onUpdate?

(`value`) => `void`

Called when data is available (may be called twice: cache then network)

#### Returns

`Promise`\<`T`\>

#### Inherited from

`BrkClientBase.getJson`

***

### getLivePrice()

> **getLivePrice**(): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7373](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7373)

Live BTC/USD price

Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.

Endpoint: `GET /api/mempool/price`

#### Returns

`Promise`\<`number`\>

***

### getMempool()

> **getMempool**(): `Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7361](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7361)

Mempool statistics

Get current mempool statistics including transaction count, total vsize, and total fees.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool/info`

#### Returns

`Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

***

### getMempoolBlocks()

> **getMempoolBlocks**(): `Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7719](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7719)

Projected mempool blocks

Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

#### Returns

`Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

***

### getMempoolTxids()

> **getMempoolTxids**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7387](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7387)

Mempool transaction IDs

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

#### Returns

`Promise`\<`string`[]\>

***

### getMetric()

> **getMetric**(`metric`, `index`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| [`AnyMetricData`](../type-aliases/AnyMetricData.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7420](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7420)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7401](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7401)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7461](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7461)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7537](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7537)

Metric count

Returns the number of metrics available per index type.

Endpoint: `GET /api/metrics/count`

#### Returns

`Promise`\<[`MetricCount`](../interfaces/MetricCount.md)[]\>

***

### getMetricsTree()

> **getMetricsTree**(): `Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7442](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7442)

Metrics catalog

Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.

Endpoint: `GET /api/metrics`

#### Returns

`Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

***

### getOpenapi()

> **getOpenapi**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7975](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7975)

OpenAPI specification

Full OpenAPI 3.1 specification for this API.

Endpoint: `GET /openapi.json`

#### Returns

`Promise`\<`any`\>

***

### getPool()

> **getPool**(`slug`): `Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7889](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7889)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7903](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7903)

List all mining pools

Get list of all known mining pools with their identifiers.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools`

#### Returns

`Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

***

### getPoolStats()

> **getPoolStats**(`time_period`): `Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7919](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7919)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7733](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7733)

Recommended fees

Get recommended fee rates for different confirmation targets based on current mempool state.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getRewardStats()

> **getRewardStats**(`block_count`): `Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7935](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7935)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7610](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7610)

Sync status

Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

Endpoint: `GET /api/server/sync`

#### Returns

`Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

***

### getText()

> **getText**(`path`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1245](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1245)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7626](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7626)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7642](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7642)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7659](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7659)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7675](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7675)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7691](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7691)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7987](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7987)

API version

Returns the current version of the API server

Endpoint: `GET /version`

#### Returns

`Promise`\<`string`\>

***

### indexToDate()

> **indexToDate**(`index`, `i`): `Date`

Defined in: [Developer/brk/modules/brk-client/index.js:5908](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L5908)

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

### isDateIndex()

> **isDateIndex**(`index`): `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:5917](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L5917)

Check if an index type is date-based.

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

#### Returns

`boolean`

***

### listMetrics()

> **listMetrics**(`page?`): `Promise`\<[`PaginatedMetrics`](../interfaces/PaginatedMetrics.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7563](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7563)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7097](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7097)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7582](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7582)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7951](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L7951)

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
