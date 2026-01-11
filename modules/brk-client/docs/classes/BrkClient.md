[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClient

# Class: BrkClient

Defined in: [Developer/brk/modules/brk-client/index.js:5136](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L5136)

Main BRK client with catalog tree and API methods

## Extends

- `BrkClientBase`

## Constructors

### Constructor

> **new BrkClient**(`options`): `BrkClient`

Defined in: [Developer/brk/modules/brk-client/index.js:6033](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L6033)

#### Parameters

##### options

`string` | [`BrkClientOptions`](../interfaces/BrkClientOptions.md)

#### Returns

`BrkClient`

#### Overrides

`BrkClientBase.constructor`

## Properties

### tree

> **tree**: [`CatalogTree`](../interfaces/CatalogTree.md)

Defined in: [Developer/brk/modules/brk-client/index.js:6036](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L6036)

## Methods

### get()

> **get**\<`T`\>(`path`, `onUpdate?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:619](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L619)

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

`BrkClientBase.get`

***

### getAddress()

> **getAddress**(`address`): `Promise`\<[`AddressStats`](../interfaces/AddressStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7444](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7444)

Address information

Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).

#### Parameters

##### address

`string`

#### Returns

`Promise`\<[`AddressStats`](../interfaces/AddressStats.md)\>

***

### getAddressTxs()

> **getAddressTxs**(`address`, `after_txid?`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7458](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7458)

Address transaction IDs

Get transaction IDs for an address, newest first. Use after_txid for pagination.

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

### getAddressTxsChain()

> **getAddressTxsChain**(`address`, `after_txid?`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7476](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7476)

Address confirmed transactions

Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.

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

### getAddressTxsMempool()

> **getAddressTxsMempool**(`address`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7494](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7494)

Address mempool transactions

Get unconfirmed transaction IDs for an address from the mempool (up to 50).

#### Parameters

##### address

`string`

#### Returns

`Promise`\<`string`[]\>

***

### getAddressUtxo()

> **getAddressUtxo**(`address`): `Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7506](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7506)

Address UTXOs

Get unspent transaction outputs for an address.

#### Parameters

##### address

`string`

#### Returns

`Promise`\<[`Utxo`](../interfaces/Utxo.md)[]\>

***

### getBlockByHash()

> **getBlockByHash**(`hash`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7530](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7530)

Block information

Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

***

### getBlockByHashRaw()

> **getBlockByHashRaw**(`hash`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7542](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7542)

Raw block

Returns the raw block data in binary format.

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<`number`[]\>

***

### getBlockByHashStatus()

> **getBlockByHashStatus**(`hash`): `Promise`\<[`BlockStatus`](../interfaces/BlockStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7554](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7554)

Block status

Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<[`BlockStatus`](../interfaces/BlockStatus.md)\>

***

### getBlockByHashTxidByIndex()

> **getBlockByHashTxidByIndex**(`hash`, `index`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7567](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7567)

Transaction ID at index

Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.

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

### getBlockByHashTxids()

> **getBlockByHashTxids**(`hash`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7579](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7579)

Block transaction IDs

Retrieve all transaction IDs in a block by block hash.

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<`string`[]\>

***

### getBlockByHashTxsByStartIndex()

> **getBlockByHashTxsByStartIndex**(`hash`, `start_index`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7592](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7592)

Block transactions (paginated)

Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.

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

### getBlockHeight()

> **getBlockHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7518](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7518)

Block by height

Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.

#### Parameters

##### height

`number`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

***

### getBlocks()

> **getBlocks**(): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7602](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7602)

Recent blocks

Retrieve the last 10 blocks. Returns block metadata for each block.

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlocksByHeight()

> **getBlocksByHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7614](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7614)

Blocks from height

Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.

#### Parameters

##### height

`number`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getHealth()

> **getHealth**(): `Promise`\<[`Health`](../interfaces/Health.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:8008](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L8008)

Health check

Returns the health status of the API server

#### Returns

`Promise`\<[`Health`](../interfaces/Health.md)\>

***

### getMempoolInfo()

> **getMempoolInfo**(): `Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7624](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7624)

Mempool statistics

Get current mempool statistics including transaction count, total vsize, and total fees.

#### Returns

`Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

***

### getMempoolTxids()

> **getMempoolTxids**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7634](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7634)

Mempool transaction IDs

Get all transaction IDs currently in the mempool.

#### Returns

`Promise`\<`string`[]\>

***

### getMetric()

> **getMetric**(`metric`): `Promise`\<[`Index`](../type-aliases/Index.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7646](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7646)

Get supported indexes for a metric

Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.

#### Parameters

##### metric

`string`

#### Returns

`Promise`\<[`Index`](../type-aliases/Index.md)[]\>

***

### getMetricByIndex()

> **getMetricByIndex**(`index`, `metric`, `count?`, `format?`, `from?`, `to?`): `Promise`\<[`AnyMetricData`](../type-aliases/AnyMetricData.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7663](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7663)

Get metric data

Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).

#### Parameters

##### index

[`Index`](../type-aliases/Index.md)

Aggregation index

##### metric

`string`

Metric name

##### count?

`any`

Number of values to return (ignored if `to` is set)

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

##### from?

`any`

Inclusive starting index, if negative counts from end

##### to?

`any`

Exclusive ending index, if negative counts from end

#### Returns

`Promise`\<[`AnyMetricData`](../type-aliases/AnyMetricData.md)\>

***

### getMetricsBulk()

> **getMetricsBulk**(`count?`, `format?`, `from?`, `index?`, `metrics?`, `to?`): `Promise`\<[`AnyMetricData`](../type-aliases/AnyMetricData.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7688](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7688)

Bulk metric data

Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects.

#### Parameters

##### count?

`any`

Number of values to return (ignored if `to` is set)

##### format?

[`Format`](../type-aliases/Format.md)

Format of the output

##### from?

`any`

Inclusive starting index, if negative counts from end

##### index?

[`Index`](../type-aliases/Index.md)

Index to query

##### metrics?

`string`

Requested metrics

##### to?

`any`

Exclusive ending index, if negative counts from end

#### Returns

`Promise`\<[`AnyMetricData`](../type-aliases/AnyMetricData.md)[]\>

***

### getMetricsCatalog()

> **getMetricsCatalog**(): `Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7706](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7706)

Metrics catalog

Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure.

#### Returns

`Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

***

### getMetricsCount()

> **getMetricsCount**(): `Promise`\<[`MetricCount`](../interfaces/MetricCount.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7716](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7716)

Metric count

Current metric count

#### Returns

`Promise`\<[`MetricCount`](../interfaces/MetricCount.md)[]\>

***

### getMetricsIndexes()

> **getMetricsIndexes**(): `Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7726](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7726)

List available indexes

Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.

#### Returns

`Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

***

### getMetricsList()

> **getMetricsList**(`page?`): `Promise`\<[`PaginatedMetrics`](../interfaces/PaginatedMetrics.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7738](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7738)

Metrics list

Paginated list of available metrics

#### Parameters

##### page?

`any`

Pagination index

#### Returns

`Promise`\<[`PaginatedMetrics`](../interfaces/PaginatedMetrics.md)\>

***

### getMetricsSearchByMetric()

> **getMetricsSearchByMetric**(`metric`, `limit?`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7754](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7754)

Search metrics

Fuzzy search for metrics by name. Supports partial matches and typos.

#### Parameters

##### metric

`string`

##### limit?

`number`

#### Returns

`Promise`\<`string`[]\>

***

### getTxByTxid()

> **getTxByTxid**(`txid`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7769](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7769)

Transaction information

Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

***

### getTxByTxidHex()

> **getTxByTxidHex**(`txid`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:7781](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7781)

Transaction hex

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<`string`\>

***

### getTxByTxidOutspendByVout()

> **getTxByTxidOutspendByVout**(`txid`, `vout`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7794](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7794)

Output spend status

Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.

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

### getTxByTxidOutspends()

> **getTxByTxidOutspends**(`txid`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7806](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7806)

All output spend statuses

Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)[]\>

***

### getTxByTxidStatus()

> **getTxByTxidStatus**(`txid`): `Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7818](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7818)

Transaction status

Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

***

### getV1DifficultyAdjustment()

> **getV1DifficultyAdjustment**(): `Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7828](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7828)

Difficulty adjustment

Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

#### Returns

`Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

***

### getV1FeesMempoolBlocks()

> **getV1FeesMempoolBlocks**(): `Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7838](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7838)

Projected mempool blocks

Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

#### Returns

`Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

***

### getV1FeesRecommended()

> **getV1FeesRecommended**(): `Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7848](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7848)

Recommended fees

Get recommended fee rates for different confirmation targets based on current mempool state.

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getV1MiningBlocksFeesByTimePeriod()

> **getV1MiningBlocksFeesByTimePeriod**(`time_period`): `Promise`\<[`BlockFeesEntry`](../interfaces/BlockFeesEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7860](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7860)

Block fees

Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`BlockFeesEntry`](../interfaces/BlockFeesEntry.md)[]\>

***

### getV1MiningBlocksRewardsByTimePeriod()

> **getV1MiningBlocksRewardsByTimePeriod**(`time_period`): `Promise`\<[`BlockRewardsEntry`](../interfaces/BlockRewardsEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7872](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7872)

Block rewards

Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`BlockRewardsEntry`](../interfaces/BlockRewardsEntry.md)[]\>

***

### getV1MiningBlocksSizesWeightsByTimePeriod()

> **getV1MiningBlocksSizesWeightsByTimePeriod**(`time_period`): `Promise`\<[`BlockSizesWeights`](../interfaces/BlockSizesWeights.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7884](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7884)

Block sizes and weights

Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`BlockSizesWeights`](../interfaces/BlockSizesWeights.md)\>

***

### getV1MiningBlocksTimestamp()

> **getV1MiningBlocksTimestamp**(`timestamp`): `Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7896](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7896)

Block by timestamp

Find the block closest to a given UNIX timestamp.

#### Parameters

##### timestamp

`number`

#### Returns

`Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

***

### getV1MiningDifficultyAdjustments()

> **getV1MiningDifficultyAdjustments**(): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7906](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7906)

Difficulty adjustments (all time)

Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent].

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getV1MiningDifficultyAdjustmentsByTimePeriod()

> **getV1MiningDifficultyAdjustmentsByTimePeriod**(`time_period`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7918](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7918)

Difficulty adjustments

Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent].

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getV1MiningHashrate()

> **getV1MiningHashrate**(): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7928](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7928)

Network hashrate (all time)

Get network hashrate and difficulty data for all time.

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getV1MiningHashrateByTimePeriod()

> **getV1MiningHashrateByTimePeriod**(`time_period`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7940](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7940)

Network hashrate

Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getV1MiningPoolBySlug()

> **getV1MiningPoolBySlug**(`slug`): `Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7952](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7952)

Mining pool details

Get detailed information about a specific mining pool including block counts and shares for different time periods.

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

#### Returns

`Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

***

### getV1MiningPools()

> **getV1MiningPools**(): `Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:7962](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7962)

List all mining pools

Get list of all known mining pools with their identifiers.

#### Returns

`Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

***

### getV1MiningPoolsByTimePeriod()

> **getV1MiningPoolsByTimePeriod**(`time_period`): `Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7974](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7974)

Mining pool statistics

Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

***

### getV1MiningRewardStatsByBlockCount()

> **getV1MiningRewardStatsByBlockCount**(`block_count`): `Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7986](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7986)

Mining reward statistics

Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.

#### Parameters

##### block\_count

`number`

Number of recent blocks to include

#### Returns

`Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

***

### getV1ValidateAddress()

> **getV1ValidateAddress**(`address`): `Promise`\<[`AddressValidation`](../interfaces/AddressValidation.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:7998](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L7998)

Validate address

Validate a Bitcoin address and get information about its type and scriptPubKey.

#### Parameters

##### address

`string`

Bitcoin address to validate (can be any string)

#### Returns

`Promise`\<[`AddressValidation`](../interfaces/AddressValidation.md)\>

***

### getVersion()

> **getVersion**(): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:8018](https://github.com/bitcoinresearchkit/brk/blob/6f45ec13f3a9e84728abdaed03e8c5432df5ffa3/modules/brk-client/index.js#L8018)

API version

Returns the current version of the API server

#### Returns

`Promise`\<`string`\>
