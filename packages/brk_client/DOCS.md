# Table of Contents

* [brk\_client](#brk_client)
  * [BrkError](#brk_client.BrkError)
  * [MetricData](#brk_client.MetricData)
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
    * [metric](#brk_client.BrkClient.metric)
    * [index\_to\_date](#brk_client.BrkClient.index_to_date)
    * [is\_date\_index](#brk_client.BrkClient.is_date_index)
    * [get\_api](#brk_client.BrkClient.get_api)
    * [get\_address](#brk_client.BrkClient.get_address)
    * [get\_address\_txs](#brk_client.BrkClient.get_address_txs)
    * [get\_address\_confirmed\_txs](#brk_client.BrkClient.get_address_confirmed_txs)
    * [get\_address\_mempool\_txs](#brk_client.BrkClient.get_address_mempool_txs)
    * [get\_address\_utxos](#brk_client.BrkClient.get_address_utxos)
    * [get\_block\_by\_height](#brk_client.BrkClient.get_block_by_height)
    * [get\_block](#brk_client.BrkClient.get_block)
    * [get\_block\_raw](#brk_client.BrkClient.get_block_raw)
    * [get\_block\_status](#brk_client.BrkClient.get_block_status)
    * [get\_block\_txid](#brk_client.BrkClient.get_block_txid)
    * [get\_block\_txids](#brk_client.BrkClient.get_block_txids)
    * [get\_block\_txs](#brk_client.BrkClient.get_block_txs)
    * [get\_blocks](#brk_client.BrkClient.get_blocks)
    * [get\_blocks\_from\_height](#brk_client.BrkClient.get_blocks_from_height)
    * [get\_mempool](#brk_client.BrkClient.get_mempool)
    * [get\_mempool\_txids](#brk_client.BrkClient.get_mempool_txids)
    * [get\_metric\_info](#brk_client.BrkClient.get_metric_info)
    * [get\_metric](#brk_client.BrkClient.get_metric)
    * [get\_metrics\_tree](#brk_client.BrkClient.get_metrics_tree)
    * [get\_metrics](#brk_client.BrkClient.get_metrics)
    * [get\_metrics\_count](#brk_client.BrkClient.get_metrics_count)
    * [get\_indexes](#brk_client.BrkClient.get_indexes)
    * [list\_metrics](#brk_client.BrkClient.list_metrics)
    * [search\_metrics](#brk_client.BrkClient.search_metrics)
    * [get\_disk\_usage](#brk_client.BrkClient.get_disk_usage)
    * [get\_sync\_status](#brk_client.BrkClient.get_sync_status)
    * [get\_tx](#brk_client.BrkClient.get_tx)
    * [get\_tx\_hex](#brk_client.BrkClient.get_tx_hex)
    * [get\_tx\_outspend](#brk_client.BrkClient.get_tx_outspend)
    * [get\_tx\_outspends](#brk_client.BrkClient.get_tx_outspends)
    * [get\_tx\_status](#brk_client.BrkClient.get_tx_status)
    * [get\_difficulty\_adjustment](#brk_client.BrkClient.get_difficulty_adjustment)
    * [get\_mempool\_blocks](#brk_client.BrkClient.get_mempool_blocks)
    * [get\_recommended\_fees](#brk_client.BrkClient.get_recommended_fees)
    * [get\_block\_fee\_rates](#brk_client.BrkClient.get_block_fee_rates)
    * [get\_block\_fees](#brk_client.BrkClient.get_block_fees)
    * [get\_block\_rewards](#brk_client.BrkClient.get_block_rewards)
    * [get\_block\_sizes\_weights](#brk_client.BrkClient.get_block_sizes_weights)
    * [get\_block\_by\_timestamp](#brk_client.BrkClient.get_block_by_timestamp)
    * [get\_difficulty\_adjustments](#brk_client.BrkClient.get_difficulty_adjustments)
    * [get\_difficulty\_adjustments\_by\_period](#brk_client.BrkClient.get_difficulty_adjustments_by_period)
    * [get\_hashrate](#brk_client.BrkClient.get_hashrate)
    * [get\_hashrate\_by\_period](#brk_client.BrkClient.get_hashrate_by_period)
    * [get\_pool](#brk_client.BrkClient.get_pool)
    * [get\_pools](#brk_client.BrkClient.get_pools)
    * [get\_pool\_stats](#brk_client.BrkClient.get_pool_stats)
    * [get\_reward\_stats](#brk_client.BrkClient.get_reward_stats)
    * [validate\_address](#brk_client.BrkClient.validate_address)
    * [get\_health](#brk_client.BrkClient.get_health)
    * [get\_openapi](#brk_client.BrkClient.get_openapi)
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
@dataclass
class MetricData(Generic[T])
```

Metric data with range information.

<a id="brk_client.BrkClient"></a>

## BrkClient Objects

```python
class BrkClient(BrkClientBase)
```

Main BRK client with metrics tree and API methods.

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

<a id="brk_client.BrkClient.metric"></a>

#### metric

```python
def metric(metric: str, index: Index) -> MetricEndpointBuilder[Any]
```

Create a dynamic metric endpoint builder for any metric/index combination.

Use this for programmatic access when the metric name is determined at runtime.
For type-safe access, use the `metrics` tree instead.

<a id="brk_client.BrkClient.index_to_date"></a>

#### index\_to\_date

```python
def index_to_date(index: Index, i: int) -> date
```

Convert an index value to a date for date-based indexes.

<a id="brk_client.BrkClient.is_date_index"></a>

#### is\_date\_index

```python
def is_date_index(index: Index) -> bool
```

Check if an index type is date-based.

<a id="brk_client.BrkClient.get_api"></a>

#### get\_api

```python
def get_api() -> Any
```

Compact OpenAPI specification.

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

<a id="brk_client.BrkClient.get_address"></a>

#### get\_address

```python
def get_address(address: Address) -> AddressStats
```

Address information.

Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*

Endpoint: `GET /api/address/{address}`

<a id="brk_client.BrkClient.get_address_txs"></a>

#### get\_address\_txs

```python
def get_address_txs(address: Address,
                    after_txid: Optional[str] = None,
                    limit: Optional[float] = None) -> List[Txid]
```

Address transaction IDs.

Get transaction IDs for an address, newest first. Use after_txid for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*

Endpoint: `GET /api/address/{address}/txs`

<a id="brk_client.BrkClient.get_address_confirmed_txs"></a>

#### get\_address\_confirmed\_txs

```python
def get_address_confirmed_txs(address: Address,
                              after_txid: Optional[str] = None,
                              limit: Optional[float] = None) -> List[Txid]
```

Address confirmed transactions.

Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*

Endpoint: `GET /api/address/{address}/txs/chain`

<a id="brk_client.BrkClient.get_address_mempool_txs"></a>

#### get\_address\_mempool\_txs

```python
def get_address_mempool_txs(address: Address) -> List[Txid]
```

Address mempool transactions.

Get unconfirmed transaction IDs for an address from the mempool (up to 50).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*

Endpoint: `GET /api/address/{address}/txs/mempool`

<a id="brk_client.BrkClient.get_address_utxos"></a>

#### get\_address\_utxos

```python
def get_address_utxos(address: Address) -> List[Utxo]
```

Address UTXOs.

Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*

Endpoint: `GET /api/address/{address}/utxo`

<a id="brk_client.BrkClient.get_block_by_height"></a>

#### get\_block\_by\_height

```python
def get_block_by_height(height: Height) -> BlockInfo
```

Block by height.

Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*

Endpoint: `GET /api/block-height/{height}`

<a id="brk_client.BrkClient.get_block"></a>

#### get\_block

```python
def get_block(hash: BlockHash) -> BlockInfo
```

Block information.

Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block)*

Endpoint: `GET /api/block/{hash}`

<a id="brk_client.BrkClient.get_block_raw"></a>

#### get\_block\_raw

```python
def get_block_raw(hash: BlockHash) -> List[float]
```

Raw block.

Returns the raw block data in binary format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-raw)*

Endpoint: `GET /api/block/{hash}/raw`

<a id="brk_client.BrkClient.get_block_status"></a>

#### get\_block\_status

```python
def get_block_status(hash: BlockHash) -> BlockStatus
```

Block status.

Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-status)*

Endpoint: `GET /api/block/{hash}/status`

<a id="brk_client.BrkClient.get_block_txid"></a>

#### get\_block\_txid

```python
def get_block_txid(hash: BlockHash, index: TxIndex) -> Txid
```

Transaction ID at index.

Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-id)*

Endpoint: `GET /api/block/{hash}/txid/{index}`

<a id="brk_client.BrkClient.get_block_txids"></a>

#### get\_block\_txids

```python
def get_block_txids(hash: BlockHash) -> List[Txid]
```

Block transaction IDs.

Retrieve all transaction IDs in a block. Returns an array of txids in block order.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-ids)*

Endpoint: `GET /api/block/{hash}/txids`

<a id="brk_client.BrkClient.get_block_txs"></a>

#### get\_block\_txs

```python
def get_block_txs(hash: BlockHash, start_index: TxIndex) -> List[Transaction]
```

Block transactions (paginated).

Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*

Endpoint: `GET /api/block/{hash}/txs/{start_index}`

<a id="brk_client.BrkClient.get_blocks"></a>

#### get\_blocks

```python
def get_blocks() -> List[BlockInfo]
```

Recent blocks.

Retrieve the last 10 blocks. Returns block metadata for each block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks`

<a id="brk_client.BrkClient.get_blocks_from_height"></a>

#### get\_blocks\_from\_height

```python
def get_blocks_from_height(height: Height) -> List[BlockInfo]
```

Blocks from height.

Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks/{height}`

<a id="brk_client.BrkClient.get_mempool"></a>

#### get\_mempool

```python
def get_mempool() -> MempoolInfo
```

Mempool statistics.

Get current mempool statistics including transaction count, total vsize, and total fees.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool/info`

<a id="brk_client.BrkClient.get_mempool_txids"></a>

#### get\_mempool\_txids

```python
def get_mempool_txids() -> List[Txid]
```

Mempool transaction IDs.

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

<a id="brk_client.BrkClient.get_metric_info"></a>

#### get\_metric\_info

```python
def get_metric_info(metric: Metric) -> List[Index]
```

Get supported indexes for a metric.

Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.

Endpoint: `GET /api/metric/{metric}`

<a id="brk_client.BrkClient.get_metric"></a>

#### get\_metric

```python
def get_metric(metric: Metric,
               index: Index,
               start: Optional[float] = None,
               end: Optional[float] = None,
               limit: Optional[str] = None,
               format: Optional[Format] = None) -> Union[AnyMetricData, str]
```

Get metric data.

Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).

Endpoint: `GET /api/metric/{metric}/{index}`

<a id="brk_client.BrkClient.get_metrics_tree"></a>

#### get\_metrics\_tree

```python
def get_metrics_tree() -> TreeNode
```

Metrics catalog.

Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.

Endpoint: `GET /api/metrics`

<a id="brk_client.BrkClient.get_metrics"></a>

#### get\_metrics

```python
def get_metrics(
        metrics: Metrics,
        index: Index,
        start: Optional[float] = None,
        end: Optional[float] = None,
        limit: Optional[str] = None,
        format: Optional[Format] = None) -> Union[List[AnyMetricData], str]
```

Bulk metric data.

Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects. For a single metric, use `get_metric` instead.

Endpoint: `GET /api/metrics/bulk`

<a id="brk_client.BrkClient.get_metrics_count"></a>

#### get\_metrics\_count

```python
def get_metrics_count() -> List[MetricCount]
```

Metric count.

Returns the number of metrics available per index type.

Endpoint: `GET /api/metrics/count`

<a id="brk_client.BrkClient.get_indexes"></a>

#### get\_indexes

```python
def get_indexes() -> List[IndexInfo]
```

List available indexes.

Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.

Endpoint: `GET /api/metrics/indexes`

<a id="brk_client.BrkClient.list_metrics"></a>

#### list\_metrics

```python
def list_metrics(page: Optional[float] = None) -> PaginatedMetrics
```

Metrics list.

Paginated flat list of all available metric names. Use `page` query param for pagination.

Endpoint: `GET /api/metrics/list`

<a id="brk_client.BrkClient.search_metrics"></a>

#### search\_metrics

```python
def search_metrics(metric: Metric,
                   limit: Optional[Limit] = None) -> List[Metric]
```

Search metrics.

Fuzzy search for metrics by name. Supports partial matches and typos.

Endpoint: `GET /api/metrics/search/{metric}`

<a id="brk_client.BrkClient.get_disk_usage"></a>

#### get\_disk\_usage

```python
def get_disk_usage() -> DiskUsage
```

Disk usage.

Returns the disk space used by BRK and Bitcoin data.

Endpoint: `GET /api/server/disk`

<a id="brk_client.BrkClient.get_sync_status"></a>

#### get\_sync\_status

```python
def get_sync_status() -> SyncStatus
```

Sync status.

Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

Endpoint: `GET /api/server/sync`

<a id="brk_client.BrkClient.get_tx"></a>

#### get\_tx

```python
def get_tx(txid: Txid) -> Transaction
```

Transaction information.

Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*

Endpoint: `GET /api/tx/{txid}`

<a id="brk_client.BrkClient.get_tx_hex"></a>

#### get\_tx\_hex

```python
def get_tx_hex(txid: Txid) -> Hex
```

Transaction hex.

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*

Endpoint: `GET /api/tx/{txid}/hex`

<a id="brk_client.BrkClient.get_tx_outspend"></a>

#### get\_tx\_outspend

```python
def get_tx_outspend(txid: Txid, vout: Vout) -> TxOutspend
```

Output spend status.

Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*

Endpoint: `GET /api/tx/{txid}/outspend/{vout}`

<a id="brk_client.BrkClient.get_tx_outspends"></a>

#### get\_tx\_outspends

```python
def get_tx_outspends(txid: Txid) -> List[TxOutspend]
```

All output spend statuses.

Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*

Endpoint: `GET /api/tx/{txid}/outspends`

<a id="brk_client.BrkClient.get_tx_status"></a>

#### get\_tx\_status

```python
def get_tx_status(txid: Txid) -> TxStatus
```

Transaction status.

Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*

Endpoint: `GET /api/tx/{txid}/status`

<a id="brk_client.BrkClient.get_difficulty_adjustment"></a>

#### get\_difficulty\_adjustment

```python
def get_difficulty_adjustment() -> DifficultyAdjustment
```

Difficulty adjustment.

Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

<a id="brk_client.BrkClient.get_mempool_blocks"></a>

#### get\_mempool\_blocks

```python
def get_mempool_blocks() -> List[MempoolBlock]
```

Projected mempool blocks.

Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

<a id="brk_client.BrkClient.get_recommended_fees"></a>

#### get\_recommended\_fees

```python
def get_recommended_fees() -> RecommendedFees
```

Recommended fees.

Get recommended fee rates for different confirmation targets based on current mempool state.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

<a id="brk_client.BrkClient.get_block_fee_rates"></a>

#### get\_block\_fee\_rates

```python
def get_block_fee_rates(time_period: TimePeriod) -> Any
```

Block fee rates (WIP).

**Work in progress.** Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-feerates)*

Endpoint: `GET /api/v1/mining/blocks/fee-rates/{time_period}`

<a id="brk_client.BrkClient.get_block_fees"></a>

#### get\_block\_fees

```python
def get_block_fees(time_period: TimePeriod) -> List[BlockFeesEntry]
```

Block fees.

Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*

Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`

<a id="brk_client.BrkClient.get_block_rewards"></a>

#### get\_block\_rewards

```python
def get_block_rewards(time_period: TimePeriod) -> List[BlockRewardsEntry]
```

Block rewards.

Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*

Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`

<a id="brk_client.BrkClient.get_block_sizes_weights"></a>

#### get\_block\_sizes\_weights

```python
def get_block_sizes_weights(time_period: TimePeriod) -> BlockSizesWeights
```

Block sizes and weights.

Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*

Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`

<a id="brk_client.BrkClient.get_block_by_timestamp"></a>

#### get\_block\_by\_timestamp

```python
def get_block_by_timestamp(timestamp: Timestamp) -> BlockTimestamp
```

Block by timestamp.

Find the block closest to a given UNIX timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*

Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`

<a id="brk_client.BrkClient.get_difficulty_adjustments"></a>

#### get\_difficulty\_adjustments

```python
def get_difficulty_adjustments() -> List[DifficultyAdjustmentEntry]
```

Difficulty adjustments (all time).

Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments`

<a id="brk_client.BrkClient.get_difficulty_adjustments_by_period"></a>

#### get\_difficulty\_adjustments\_by\_period

```python
def get_difficulty_adjustments_by_period(
        time_period: TimePeriod) -> List[DifficultyAdjustmentEntry]
```

Difficulty adjustments.

Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`

<a id="brk_client.BrkClient.get_hashrate"></a>

#### get\_hashrate

```python
def get_hashrate() -> HashrateSummary
```

Network hashrate (all time).

Get network hashrate and difficulty data for all time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate`

<a id="brk_client.BrkClient.get_hashrate_by_period"></a>

#### get\_hashrate\_by\_period

```python
def get_hashrate_by_period(time_period: TimePeriod) -> HashrateSummary
```

Network hashrate.

Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate/{time_period}`

<a id="brk_client.BrkClient.get_pool"></a>

#### get\_pool

```python
def get_pool(slug: PoolSlug) -> PoolDetail
```

Mining pool details.

Get detailed information about a specific mining pool including block counts and shares for different time periods.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool)*

Endpoint: `GET /api/v1/mining/pool/{slug}`

<a id="brk_client.BrkClient.get_pools"></a>

#### get\_pools

```python
def get_pools() -> List[PoolInfo]
```

List all mining pools.

Get list of all known mining pools with their identifiers.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools`

<a id="brk_client.BrkClient.get_pool_stats"></a>

#### get\_pool\_stats

```python
def get_pool_stats(time_period: TimePeriod) -> PoolsSummary
```

Mining pool statistics.

Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools/{time_period}`

<a id="brk_client.BrkClient.get_reward_stats"></a>

#### get\_reward\_stats

```python
def get_reward_stats(block_count: float) -> RewardStats
```

Mining reward statistics.

Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*

Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`

<a id="brk_client.BrkClient.validate_address"></a>

#### validate\_address

```python
def validate_address(address: str) -> AddressValidation
```

Validate address.

Validate a Bitcoin address and get information about its type and scriptPubKey.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*

Endpoint: `GET /api/v1/validate-address/{address}`

<a id="brk_client.BrkClient.get_health"></a>

#### get\_health

```python
def get_health() -> Health
```

Health check.

Returns the health status of the API server, including uptime information.

Endpoint: `GET /health`

<a id="brk_client.BrkClient.get_openapi"></a>

#### get\_openapi

```python
def get_openapi() -> Any
```

OpenAPI specification.

Full OpenAPI 3.1 specification for this API.

Endpoint: `GET /openapi.json`

<a id="brk_client.BrkClient.get_version"></a>

#### get\_version

```python
def get_version() -> str
```

API version.

Returns the current version of the API server

Endpoint: `GET /version`

