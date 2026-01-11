# Table of Contents

* [brk\_client](#brk_client)
  * [BrkError](#brk_client.BrkError)
  * [MetricData](#brk_client.MetricData)
  * [MetricEndpoint](#brk_client.MetricEndpoint)
  * [BrkClient](#brk_client.BrkClient)
    * [VERSION](#brk_client.BrkClient.VERSION)
    * [INDEXES](#brk_client.BrkClient.INDEXES)
    * [POOL\_ID\_TO\_POOL\_NAME](#brk_client.BrkClient.POOL_ID_TO_POOL_NAME)
    * [TERM\_NAMES](#brk_client.BrkClient.TERM_NAMES)
    * [EPOCH\_NAMES](#brk_client.BrkClient.EPOCH_NAMES)
    * [YEAR\_NAMES](#brk_client.BrkClient.YEAR_NAMES)
    * [SPENDABLE\_TYPE\_NAMES](#brk_client.BrkClient.SPENDABLE_TYPE_NAMES)
    * [AGE\_RANGE\_NAMES](#brk_client.BrkClient.AGE_RANGE_NAMES)
    * [MAX\_AGE\_NAMES](#brk_client.BrkClient.MAX_AGE_NAMES)
    * [MIN\_AGE\_NAMES](#brk_client.BrkClient.MIN_AGE_NAMES)
    * [AMOUNT\_RANGE\_NAMES](#brk_client.BrkClient.AMOUNT_RANGE_NAMES)
    * [GE\_AMOUNT\_NAMES](#brk_client.BrkClient.GE_AMOUNT_NAMES)
    * [LT\_AMOUNT\_NAMES](#brk_client.BrkClient.LT_AMOUNT_NAMES)
    * [\_\_init\_\_](#brk_client.BrkClient.__init__)
    * [get\_address](#brk_client.BrkClient.get_address)
    * [get\_address\_txs](#brk_client.BrkClient.get_address_txs)
    * [get\_address\_txs\_chain](#brk_client.BrkClient.get_address_txs_chain)
    * [get\_address\_txs\_mempool](#brk_client.BrkClient.get_address_txs_mempool)
    * [get\_address\_utxo](#brk_client.BrkClient.get_address_utxo)
    * [get\_block\_height](#brk_client.BrkClient.get_block_height)
    * [get\_block\_by\_hash](#brk_client.BrkClient.get_block_by_hash)
    * [get\_block\_by\_hash\_raw](#brk_client.BrkClient.get_block_by_hash_raw)
    * [get\_block\_by\_hash\_status](#brk_client.BrkClient.get_block_by_hash_status)
    * [get\_block\_by\_hash\_txid\_by\_index](#brk_client.BrkClient.get_block_by_hash_txid_by_index)
    * [get\_block\_by\_hash\_txids](#brk_client.BrkClient.get_block_by_hash_txids)
    * [get\_block\_by\_hash\_txs\_by\_start\_index](#brk_client.BrkClient.get_block_by_hash_txs_by_start_index)
    * [get\_blocks](#brk_client.BrkClient.get_blocks)
    * [get\_blocks\_by\_height](#brk_client.BrkClient.get_blocks_by_height)
    * [get\_mempool\_info](#brk_client.BrkClient.get_mempool_info)
    * [get\_mempool\_txids](#brk_client.BrkClient.get_mempool_txids)
    * [get\_metric](#brk_client.BrkClient.get_metric)
    * [get\_metric\_by\_index](#brk_client.BrkClient.get_metric_by_index)
    * [get\_metrics\_bulk](#brk_client.BrkClient.get_metrics_bulk)
    * [get\_metrics\_catalog](#brk_client.BrkClient.get_metrics_catalog)
    * [get\_metrics\_count](#brk_client.BrkClient.get_metrics_count)
    * [get\_metrics\_indexes](#brk_client.BrkClient.get_metrics_indexes)
    * [get\_metrics\_list](#brk_client.BrkClient.get_metrics_list)
    * [get\_metrics\_search\_by\_metric](#brk_client.BrkClient.get_metrics_search_by_metric)
    * [get\_tx\_by\_txid](#brk_client.BrkClient.get_tx_by_txid)
    * [get\_tx\_by\_txid\_hex](#brk_client.BrkClient.get_tx_by_txid_hex)
    * [get\_tx\_by\_txid\_outspend\_by\_vout](#brk_client.BrkClient.get_tx_by_txid_outspend_by_vout)
    * [get\_tx\_by\_txid\_outspends](#brk_client.BrkClient.get_tx_by_txid_outspends)
    * [get\_tx\_by\_txid\_status](#brk_client.BrkClient.get_tx_by_txid_status)
    * [get\_v1\_difficulty\_adjustment](#brk_client.BrkClient.get_v1_difficulty_adjustment)
    * [get\_v1\_fees\_mempool\_blocks](#brk_client.BrkClient.get_v1_fees_mempool_blocks)
    * [get\_v1\_fees\_recommended](#brk_client.BrkClient.get_v1_fees_recommended)
    * [get\_v1\_mining\_blocks\_fees\_by\_time\_period](#brk_client.BrkClient.get_v1_mining_blocks_fees_by_time_period)
    * [get\_v1\_mining\_blocks\_rewards\_by\_time\_period](#brk_client.BrkClient.get_v1_mining_blocks_rewards_by_time_period)
    * [get\_v1\_mining\_blocks\_sizes\_weights\_by\_time\_period](#brk_client.BrkClient.get_v1_mining_blocks_sizes_weights_by_time_period)
    * [get\_v1\_mining\_blocks\_timestamp](#brk_client.BrkClient.get_v1_mining_blocks_timestamp)
    * [get\_v1\_mining\_difficulty\_adjustments](#brk_client.BrkClient.get_v1_mining_difficulty_adjustments)
    * [get\_v1\_mining\_difficulty\_adjustments\_by\_time\_period](#brk_client.BrkClient.get_v1_mining_difficulty_adjustments_by_time_period)
    * [get\_v1\_mining\_hashrate](#brk_client.BrkClient.get_v1_mining_hashrate)
    * [get\_v1\_mining\_hashrate\_by\_time\_period](#brk_client.BrkClient.get_v1_mining_hashrate_by_time_period)
    * [get\_v1\_mining\_pool\_by\_slug](#brk_client.BrkClient.get_v1_mining_pool_by_slug)
    * [get\_v1\_mining\_pools](#brk_client.BrkClient.get_v1_mining_pools)
    * [get\_v1\_mining\_pools\_by\_time\_period](#brk_client.BrkClient.get_v1_mining_pools_by_time_period)
    * [get\_v1\_mining\_reward\_stats\_by\_block\_count](#brk_client.BrkClient.get_v1_mining_reward_stats_by_block_count)
    * [get\_v1\_validate\_address](#brk_client.BrkClient.get_v1_validate_address)
    * [get\_health](#brk_client.BrkClient.get_health)
    * [get\_version](#brk_client.BrkClient.get_version)

<a id="brk_client"></a>

# brk\_client

<a id="brk_client.BrkError"></a>

## BrkError Objects

```python
class BrkError(Exception)
```

Custom error class for BRK client errors.

<a id="brk_client.MetricData"></a>

## MetricData Objects

```python
class MetricData(TypedDict, Generic[T])
```

Metric data with range information.

<a id="brk_client.MetricEndpoint"></a>

## MetricEndpoint Objects

```python
class MetricEndpoint(Generic[T])
```

An endpoint for a specific metric + index combination.

<a id="brk_client.BrkClient"></a>

## BrkClient Objects

```python
class BrkClient(BrkClientBase)
```

Main BRK client with catalog tree and API methods.

<a id="brk_client.BrkClient.VERSION"></a>

#### VERSION

<a id="brk_client.BrkClient.INDEXES"></a>

#### INDEXES

<a id="brk_client.BrkClient.POOL_ID_TO_POOL_NAME"></a>

#### POOL\_ID\_TO\_POOL\_NAME

<a id="brk_client.BrkClient.TERM_NAMES"></a>

#### TERM\_NAMES

<a id="brk_client.BrkClient.EPOCH_NAMES"></a>

#### EPOCH\_NAMES

<a id="brk_client.BrkClient.YEAR_NAMES"></a>

#### YEAR\_NAMES

<a id="brk_client.BrkClient.SPENDABLE_TYPE_NAMES"></a>

#### SPENDABLE\_TYPE\_NAMES

<a id="brk_client.BrkClient.AGE_RANGE_NAMES"></a>

#### AGE\_RANGE\_NAMES

<a id="brk_client.BrkClient.MAX_AGE_NAMES"></a>

#### MAX\_AGE\_NAMES

<a id="brk_client.BrkClient.MIN_AGE_NAMES"></a>

#### MIN\_AGE\_NAMES

<a id="brk_client.BrkClient.AMOUNT_RANGE_NAMES"></a>

#### AMOUNT\_RANGE\_NAMES

<a id="brk_client.BrkClient.GE_AMOUNT_NAMES"></a>

#### GE\_AMOUNT\_NAMES

<a id="brk_client.BrkClient.LT_AMOUNT_NAMES"></a>

#### LT\_AMOUNT\_NAMES

<a id="brk_client.BrkClient.__init__"></a>

#### \_\_init\_\_

```python
def __init__(base_url: str = 'http://localhost:3000', timeout: float = 30.0)
```

<a id="brk_client.BrkClient.get_address"></a>

#### get\_address

```python
def get_address(address: Address) -> AddressStats
```

Address information.

Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).

<a id="brk_client.BrkClient.get_address_txs"></a>

#### get\_address\_txs

```python
def get_address_txs(address: Address,
                    after_txid: Optional[str] = None,
                    limit: Optional[int] = None) -> List[Txid]
```

Address transaction IDs.

Get transaction IDs for an address, newest first. Use after_txid for pagination.

<a id="brk_client.BrkClient.get_address_txs_chain"></a>

#### get\_address\_txs\_chain

```python
def get_address_txs_chain(address: Address,
                          after_txid: Optional[str] = None,
                          limit: Optional[int] = None) -> List[Txid]
```

Address confirmed transactions.

Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.

<a id="brk_client.BrkClient.get_address_txs_mempool"></a>

#### get\_address\_txs\_mempool

```python
def get_address_txs_mempool(address: Address) -> List[Txid]
```

Address mempool transactions.

Get unconfirmed transaction IDs for an address from the mempool (up to 50).

<a id="brk_client.BrkClient.get_address_utxo"></a>

#### get\_address\_utxo

```python
def get_address_utxo(address: Address) -> List[Utxo]
```

Address UTXOs.

Get unspent transaction outputs for an address.

<a id="brk_client.BrkClient.get_block_height"></a>

#### get\_block\_height

```python
def get_block_height(height: Height) -> BlockInfo
```

Block by height.

Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.

<a id="brk_client.BrkClient.get_block_by_hash"></a>

#### get\_block\_by\_hash

```python
def get_block_by_hash(hash: BlockHash) -> BlockInfo
```

Block information.

Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.

<a id="brk_client.BrkClient.get_block_by_hash_raw"></a>

#### get\_block\_by\_hash\_raw

```python
def get_block_by_hash_raw(hash: BlockHash) -> List[int]
```

Raw block.

Returns the raw block data in binary format.

<a id="brk_client.BrkClient.get_block_by_hash_status"></a>

#### get\_block\_by\_hash\_status

```python
def get_block_by_hash_status(hash: BlockHash) -> BlockStatus
```

Block status.

Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.

<a id="brk_client.BrkClient.get_block_by_hash_txid_by_index"></a>

#### get\_block\_by\_hash\_txid\_by\_index

```python
def get_block_by_hash_txid_by_index(hash: BlockHash, index: TxIndex) -> Txid
```

Transaction ID at index.

Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.

<a id="brk_client.BrkClient.get_block_by_hash_txids"></a>

#### get\_block\_by\_hash\_txids

```python
def get_block_by_hash_txids(hash: BlockHash) -> List[Txid]
```

Block transaction IDs.

Retrieve all transaction IDs in a block by block hash.

<a id="brk_client.BrkClient.get_block_by_hash_txs_by_start_index"></a>

#### get\_block\_by\_hash\_txs\_by\_start\_index

```python
def get_block_by_hash_txs_by_start_index(
        hash: BlockHash, start_index: TxIndex) -> List[Transaction]
```

Block transactions (paginated).

Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.

<a id="brk_client.BrkClient.get_blocks"></a>

#### get\_blocks

```python
def get_blocks() -> List[BlockInfo]
```

Recent blocks.

Retrieve the last 10 blocks. Returns block metadata for each block.

<a id="brk_client.BrkClient.get_blocks_by_height"></a>

#### get\_blocks\_by\_height

```python
def get_blocks_by_height(height: Height) -> List[BlockInfo]
```

Blocks from height.

Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.

<a id="brk_client.BrkClient.get_mempool_info"></a>

#### get\_mempool\_info

```python
def get_mempool_info() -> MempoolInfo
```

Mempool statistics.

Get current mempool statistics including transaction count, total vsize, and total fees.

<a id="brk_client.BrkClient.get_mempool_txids"></a>

#### get\_mempool\_txids

```python
def get_mempool_txids() -> List[Txid]
```

Mempool transaction IDs.

Get all transaction IDs currently in the mempool.

<a id="brk_client.BrkClient.get_metric"></a>

#### get\_metric

```python
def get_metric(metric: Metric) -> List[Index]
```

Get supported indexes for a metric.

Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.

<a id="brk_client.BrkClient.get_metric_by_index"></a>

#### get\_metric\_by\_index

```python
def get_metric_by_index(index: Index,
                        metric: Metric,
                        count: Optional[Any] = None,
                        format: Optional[Format] = None,
                        from_: Optional[Any] = None,
                        to: Optional[Any] = None) -> AnyMetricData
```

Get metric data.

Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).

<a id="brk_client.BrkClient.get_metrics_bulk"></a>

#### get\_metrics\_bulk

```python
def get_metrics_bulk(index: Index,
                     metrics: Metrics,
                     count: Optional[Any] = None,
                     format: Optional[Format] = None,
                     from_: Optional[Any] = None,
                     to: Optional[Any] = None) -> List[AnyMetricData]
```

Bulk metric data.

Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects.

<a id="brk_client.BrkClient.get_metrics_catalog"></a>

#### get\_metrics\_catalog

```python
def get_metrics_catalog() -> TreeNode
```

Metrics catalog.

Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure.

<a id="brk_client.BrkClient.get_metrics_count"></a>

#### get\_metrics\_count

```python
def get_metrics_count() -> List[MetricCount]
```

Metric count.

Current metric count

<a id="brk_client.BrkClient.get_metrics_indexes"></a>

#### get\_metrics\_indexes

```python
def get_metrics_indexes() -> List[IndexInfo]
```

List available indexes.

Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.

<a id="brk_client.BrkClient.get_metrics_list"></a>

#### get\_metrics\_list

```python
def get_metrics_list(page: Optional[Any] = None) -> PaginatedMetrics
```

Metrics list.

Paginated list of available metrics

<a id="brk_client.BrkClient.get_metrics_search_by_metric"></a>

#### get\_metrics\_search\_by\_metric

```python
def get_metrics_search_by_metric(metric: Metric,
                                 limit: Optional[Limit] = None
                                 ) -> List[Metric]
```

Search metrics.

Fuzzy search for metrics by name. Supports partial matches and typos.

<a id="brk_client.BrkClient.get_tx_by_txid"></a>

#### get\_tx\_by\_txid

```python
def get_tx_by_txid(txid: Txid) -> Transaction
```

Transaction information.

Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.

<a id="brk_client.BrkClient.get_tx_by_txid_hex"></a>

#### get\_tx\_by\_txid\_hex

```python
def get_tx_by_txid_hex(txid: Txid) -> Hex
```

Transaction hex.

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

<a id="brk_client.BrkClient.get_tx_by_txid_outspend_by_vout"></a>

#### get\_tx\_by\_txid\_outspend\_by\_vout

```python
def get_tx_by_txid_outspend_by_vout(txid: Txid, vout: Vout) -> TxOutspend
```

Output spend status.

Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.

<a id="brk_client.BrkClient.get_tx_by_txid_outspends"></a>

#### get\_tx\_by\_txid\_outspends

```python
def get_tx_by_txid_outspends(txid: Txid) -> List[TxOutspend]
```

All output spend statuses.

Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.

<a id="brk_client.BrkClient.get_tx_by_txid_status"></a>

#### get\_tx\_by\_txid\_status

```python
def get_tx_by_txid_status(txid: Txid) -> TxStatus
```

Transaction status.

Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

<a id="brk_client.BrkClient.get_v1_difficulty_adjustment"></a>

#### get\_v1\_difficulty\_adjustment

```python
def get_v1_difficulty_adjustment() -> DifficultyAdjustment
```

Difficulty adjustment.

Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

<a id="brk_client.BrkClient.get_v1_fees_mempool_blocks"></a>

#### get\_v1\_fees\_mempool\_blocks

```python
def get_v1_fees_mempool_blocks() -> List[MempoolBlock]
```

Projected mempool blocks.

Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

<a id="brk_client.BrkClient.get_v1_fees_recommended"></a>

#### get\_v1\_fees\_recommended

```python
def get_v1_fees_recommended() -> RecommendedFees
```

Recommended fees.

Get recommended fee rates for different confirmation targets based on current mempool state.

<a id="brk_client.BrkClient.get_v1_mining_blocks_fees_by_time_period"></a>

#### get\_v1\_mining\_blocks\_fees\_by\_time\_period

```python
def get_v1_mining_blocks_fees_by_time_period(
        time_period: TimePeriod) -> List[BlockFeesEntry]
```

Block fees.

Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

<a id="brk_client.BrkClient.get_v1_mining_blocks_rewards_by_time_period"></a>

#### get\_v1\_mining\_blocks\_rewards\_by\_time\_period

```python
def get_v1_mining_blocks_rewards_by_time_period(
        time_period: TimePeriod) -> List[BlockRewardsEntry]
```

Block rewards.

Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

<a id="brk_client.BrkClient.get_v1_mining_blocks_sizes_weights_by_time_period"></a>

#### get\_v1\_mining\_blocks\_sizes\_weights\_by\_time\_period

```python
def get_v1_mining_blocks_sizes_weights_by_time_period(
        time_period: TimePeriod) -> BlockSizesWeights
```

Block sizes and weights.

Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

<a id="brk_client.BrkClient.get_v1_mining_blocks_timestamp"></a>

#### get\_v1\_mining\_blocks\_timestamp

```python
def get_v1_mining_blocks_timestamp(timestamp: Timestamp) -> BlockTimestamp
```

Block by timestamp.

Find the block closest to a given UNIX timestamp.

<a id="brk_client.BrkClient.get_v1_mining_difficulty_adjustments"></a>

#### get\_v1\_mining\_difficulty\_adjustments

```python
def get_v1_mining_difficulty_adjustments() -> List[DifficultyAdjustmentEntry]
```

Difficulty adjustments (all time).

Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent].

<a id="brk_client.BrkClient.get_v1_mining_difficulty_adjustments_by_time_period"></a>

#### get\_v1\_mining\_difficulty\_adjustments\_by\_time\_period

```python
def get_v1_mining_difficulty_adjustments_by_time_period(
        time_period: TimePeriod) -> List[DifficultyAdjustmentEntry]
```

Difficulty adjustments.

Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent].

<a id="brk_client.BrkClient.get_v1_mining_hashrate"></a>

#### get\_v1\_mining\_hashrate

```python
def get_v1_mining_hashrate() -> HashrateSummary
```

Network hashrate (all time).

Get network hashrate and difficulty data for all time.

<a id="brk_client.BrkClient.get_v1_mining_hashrate_by_time_period"></a>

#### get\_v1\_mining\_hashrate\_by\_time\_period

```python
def get_v1_mining_hashrate_by_time_period(
        time_period: TimePeriod) -> HashrateSummary
```

Network hashrate.

Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

<a id="brk_client.BrkClient.get_v1_mining_pool_by_slug"></a>

#### get\_v1\_mining\_pool\_by\_slug

```python
def get_v1_mining_pool_by_slug(slug: PoolSlug) -> PoolDetail
```

Mining pool details.

Get detailed information about a specific mining pool including block counts and shares for different time periods.

<a id="brk_client.BrkClient.get_v1_mining_pools"></a>

#### get\_v1\_mining\_pools

```python
def get_v1_mining_pools() -> List[PoolInfo]
```

List all mining pools.

Get list of all known mining pools with their identifiers.

<a id="brk_client.BrkClient.get_v1_mining_pools_by_time_period"></a>

#### get\_v1\_mining\_pools\_by\_time\_period

```python
def get_v1_mining_pools_by_time_period(
        time_period: TimePeriod) -> PoolsSummary
```

Mining pool statistics.

Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

<a id="brk_client.BrkClient.get_v1_mining_reward_stats_by_block_count"></a>

#### get\_v1\_mining\_reward\_stats\_by\_block\_count

```python
def get_v1_mining_reward_stats_by_block_count(block_count: int) -> RewardStats
```

Mining reward statistics.

Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.

<a id="brk_client.BrkClient.get_v1_validate_address"></a>

#### get\_v1\_validate\_address

```python
def get_v1_validate_address(address: str) -> AddressValidation
```

Validate address.

Validate a Bitcoin address and get information about its type and scriptPubKey.

<a id="brk_client.BrkClient.get_health"></a>

#### get\_health

```python
def get_health() -> Health
```

Health check.

Returns the health status of the API server

<a id="brk_client.BrkClient.get_version"></a>

#### get\_version

```python
def get_version() -> str
```

API version.

Returns the current version of the API server

