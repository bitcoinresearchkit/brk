[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClient

# Class: BrkClient

Defined in: [Developer/brk/modules/brk-client/index.js:6359](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L6359)

Main BRK client with series tree and API methods

## Extends

- `BrkClientBase`

## Constructors

### Constructor

> **new BrkClient**(`options`): `BrkClient`

Defined in: [Developer/brk/modules/brk-client/index.js:7536](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L7536)

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

Defined in: [Developer/brk/modules/brk-client/index.js:1347](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1347)

#### Inherited from

`BrkClientBase._cache`

***

### \_cachePromise

> **\_cachePromise**: `Promise`\<`Cache` \| `null`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1345](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1345)

#### Inherited from

`BrkClientBase._cachePromise`

***

### series

> **series**: [`SeriesTree`](../interfaces/SeriesTree.md)

Defined in: [Developer/brk/modules/brk-client/index.js:7539](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L7539)

## Methods

### \_fetchSeriesData()

> **\_fetchSeriesData**\<`T`\>(`path`, `onUpdate?`): `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1435](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1435)

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

### dateToIndex()

> **dateToIndex**(`index`, `d`): `number`

Defined in: [Developer/brk/modules/brk-client/index.js:7528](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L7528)

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

> **get**(`path`): `Promise`\<`Response`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1355](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1355)

#### Parameters

##### path

`string`

#### Returns

`Promise`\<`Response`\>

#### Inherited from

`BrkClientBase.get`

***

### getAddress()

> **getAddress**(`address`): `Promise`\<[`AddrStats`](../interfaces/AddrStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9224](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9224)

Address information

Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*

Endpoint: `GET /api/address/{address}`

#### Parameters

##### address

`string`

#### Returns

`Promise`\<[`AddrStats`](../interfaces/AddrStats.md)\>

***

### getAddressConfirmedTxs()

> **getAddressConfirmedTxs**(`address`, `after_txid?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9262](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9262)

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

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getAddressMempoolTxs()

> **getAddressMempoolTxs**(`address`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9282](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9282)

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

> **getAddressTxs**(`address`, `after_txid?`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9241](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9241)

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

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getAddressUtxos()

> **getAddressUtxos**(`address`): `Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9298](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9298)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9208](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9208)

Compact OpenAPI specification

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

#### Returns

`Promise`\<`any`\>

***

### getBlock()

> **getBlock**(`hash`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9330](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9330)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9314](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9314)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9985](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9985)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9921](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9921)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9937](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9937)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9346](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9346)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9953](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9953)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9426](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9426)

Recent blocks

Retrieve the last 10 blocks. Returns block metadata for each block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlocksFromHeight()

> **getBlocksFromHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9442](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9442)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9969](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9969)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9362](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9362)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9379](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9379)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9395](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9395)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9412](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9412)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9572](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9572)

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

#### Returns

`Promise`\<`Object`\>

***

### getCostBasisCohorts()

> **getCostBasisCohorts**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9537](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9537)

Available cost basis cohorts

List available cohorts for cost basis distribution.

Endpoint: `GET /api/series/cost-basis`

#### Returns

`Promise`\<`string`[]\>

***

### getCostBasisDates()

> **getCostBasisDates**(`cohort`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9551](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9551)

Available cost basis dates

List available dates for a cohort's cost basis distribution.

Endpoint: `GET /api/series/cost-basis/{cohort}/dates`

#### Parameters

##### cohort

`string`

#### Returns

`Promise`\<`number`[]\>

***

### getDifficultyAdjustment()

> **getDifficultyAdjustment**(): `Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9877](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9877)

Difficulty adjustment

Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

#### Returns

`Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

***

### getDifficultyAdjustments()

> **getDifficultyAdjustments**(): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9999](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9999)

Difficulty adjustments (all time)

Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments`

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDifficultyAdjustmentsByPeriod()

> **getDifficultyAdjustmentsByPeriod**(`time_period`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10015](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10015)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9770](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9770)

Disk usage

Returns the disk space used by BRK and Bitcoin data.

Endpoint: `GET /api/server/disk`

#### Returns

`Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

***

### getHashrate()

> **getHashrate**(): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10029](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10029)

Network hashrate (all time)

Get network hashrate and difficulty data for all time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate`

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHashrateByPeriod()

> **getHashrateByPeriod**(`time_period`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10045](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10045)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10135](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10135)

Health check

Returns the health status of the API server, including uptime information.

Endpoint: `GET /health`

#### Returns

`Promise`\<[`Health`](../interfaces/Health.md)\>

***

### getIndexes()

> **getIndexes**(): `Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9601](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9601)

List available indexes

Returns all available indexes with their accepted query aliases. Use any alias when querying series.

Endpoint: `GET /api/series/indexes`

#### Returns

`Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

***

### getJson()

> **getJson**\<`T`\>(`path`, `onUpdate?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1369](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1369)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9468](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9468)

Live BTC/USD price

Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.

Endpoint: `GET /api/mempool/price`

#### Returns

`Promise`\<`number`\>

***

### getMempool()

> **getMempool**(): `Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9456](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9456)

Mempool statistics

Get current mempool statistics including transaction count, total vsize, and total fees.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool/info`

#### Returns

`Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

***

### getMempoolBlocks()

> **getMempoolBlocks**(): `Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9891](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9891)

Projected mempool blocks

Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

#### Returns

`Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

***

### getMempoolTxids()

> **getMempoolTxids**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9482](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9482)

Mempool transaction IDs

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

#### Returns

`Promise`\<`string`[]\>

***

### getOpenapi()

> **getOpenapi**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10147](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10147)

OpenAPI specification

Full OpenAPI 3.1 specification for this API.

Endpoint: `GET /openapi.json`

#### Returns

`Promise`\<`any`\>

***

### getPool()

> **getPool**(`slug`): `Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10061](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10061)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10075](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10075)

List all mining pools

Get list of all known mining pools with their identifiers.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools`

#### Returns

`Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

***

### getPoolStats()

> **getPoolStats**(`time_period`): `Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10091](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10091)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9905](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9905)

Recommended fees

Get recommended fee rates for different confirmation targets based on current mempool state.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getRewardStats()

> **getRewardStats**(`block_count`): `Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10107](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10107)

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

### getSeries()

> **getSeries**(`series`, `index`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9674](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9674)

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

#### Returns

`Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)\>

***

### getSeriesBulk()

> **getSeriesBulk**(`series?`, `index?`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9513](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9513)

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

#### Returns

`Promise`\<`string` \| [`AnySeriesData`](../type-aliases/AnySeriesData.md)[]\>

***

### getSeriesCount()

> **getSeriesCount**(): `Promise`\<[`SeriesCount`](../interfaces/SeriesCount.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9589](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9589)

Series count

Returns the number of series available per index type.

Endpoint: `GET /api/series/count`

#### Returns

`Promise`\<[`SeriesCount`](../interfaces/SeriesCount.md)[]\>

***

### getSeriesData()

> **getSeriesData**(`series`, `index`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| `boolean`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9703](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9703)

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

#### Returns

`Promise`\<`string` \| `boolean`[]\>

***

### getSeriesInfo()

> **getSeriesInfo**(`series`): `Promise`\<[`SeriesInfo`](../interfaces/SeriesInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9655](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9655)

Get series info

Returns the supported indexes and value type for the specified series.

Endpoint: `GET /api/series/{series}`

#### Parameters

##### series

`string`

#### Returns

`Promise`\<[`SeriesInfo`](../interfaces/SeriesInfo.md)\>

***

### getSeriesLatest()

> **getSeriesLatest**(`series`, `index`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9728](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9728)

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

#### Returns

`Promise`\<`any`\>

***

### getSeriesLen()

> **getSeriesLen**(`series`, `index`): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9743](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9743)

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

#### Returns

`Promise`\<`number`\>

***

### getSeriesTree()

> **getSeriesTree**(): `Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9494](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9494)

Series catalog

Returns the complete hierarchical catalog of available series organized as a tree structure. Series are grouped by categories and subcategories.

Endpoint: `GET /api/series`

#### Returns

`Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

***

### getSeriesVersion()

> **getSeriesVersion**(`series`, `index`): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9758](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9758)

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

#### Returns

`Promise`\<`number`\>

***

### getSyncStatus()

> **getSyncStatus**(): `Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9782](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9782)

Sync status

Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

Endpoint: `GET /api/server/sync`

#### Returns

`Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

***

### getText()

> **getText**(`path`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1423](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L1423)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9798](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9798)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9814](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9814)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9831](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9831)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9847](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9847)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9863](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9863)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10159](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10159)

API version

Returns the current version of the API server

Endpoint: `GET /version`

#### Returns

`Promise`\<`string`\>

***

### indexToDate()

> **indexToDate**(`index`, `i`): `Date`

Defined in: [Developer/brk/modules/brk-client/index.js:7518](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L7518)

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

> **listSeries**(`page?`, `per_page?`): `Promise`\<[`PaginatedSeries`](../interfaces/PaginatedSeries.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9616](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9616)

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

#### Returns

`Promise`\<[`PaginatedSeries`](../interfaces/PaginatedSeries.md)\>

***

### searchSeries()

> **searchSeries**(`q?`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9636](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9636)

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

#### Returns

`Promise`\<`string`[]\>

***

### seriesEndpoint()

> **seriesEndpoint**(`series`, `index`): [`SeriesEndpoint`](../interfaces/SeriesEndpoint.md)\<`unknown`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9196](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L9196)

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

> **validateAddress**(`address`): `Promise`\<[`AddrValidation`](../interfaces/AddrValidation.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10123](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L10123)

Validate address

Validate a Bitcoin address and get information about its type and scriptPubKey.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*

Endpoint: `GET /api/v1/validate-address/{address}`

#### Parameters

##### address

`string`

Bitcoin address to validate (can be any string)

#### Returns

`Promise`\<[`AddrValidation`](../interfaces/AddrValidation.md)\>
