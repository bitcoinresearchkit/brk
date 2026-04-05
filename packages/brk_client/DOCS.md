# Table of Contents

* [brk\_client](#brk_client)
  * [BrkError](#brk_client.BrkError)
  * [BrkClient](#brk_client.BrkClient)
    * [VERSION](#brk_client.BrkClient.VERSION)
    * [INDEXES](#brk_client.BrkClient.INDEXES)
    * [POOL\_ID\_TO\_POOL\_NAME](#brk_client.BrkClient.POOL_ID_TO_POOL_NAME)
    * [TERM\_NAMES](#brk_client.BrkClient.TERM_NAMES)
    * [EPOCH\_NAMES](#brk_client.BrkClient.EPOCH_NAMES)
    * [CLASS\_NAMES](#brk_client.BrkClient.CLASS_NAMES)
    * [SPENDABLE\_TYPE\_NAMES](#brk_client.BrkClient.SPENDABLE_TYPE_NAMES)
    * [AGE\_RANGE\_NAMES](#brk_client.BrkClient.AGE_RANGE_NAMES)
    * [UNDER\_AGE\_NAMES](#brk_client.BrkClient.UNDER_AGE_NAMES)
    * [OVER\_AGE\_NAMES](#brk_client.BrkClient.OVER_AGE_NAMES)
    * [AMOUNT\_RANGE\_NAMES](#brk_client.BrkClient.AMOUNT_RANGE_NAMES)
    * [OVER\_AMOUNT\_NAMES](#brk_client.BrkClient.OVER_AMOUNT_NAMES)
    * [UNDER\_AMOUNT\_NAMES](#brk_client.BrkClient.UNDER_AMOUNT_NAMES)
    * [PROFITABILITY\_RANGE\_NAMES](#brk_client.BrkClient.PROFITABILITY_RANGE_NAMES)
    * [PROFIT\_NAMES](#brk_client.BrkClient.PROFIT_NAMES)
    * [LOSS\_NAMES](#brk_client.BrkClient.LOSS_NAMES)
    * [\_\_init\_\_](#brk_client.BrkClient.__init__)
    * [series\_endpoint](#brk_client.BrkClient.series_endpoint)
    * [index\_to\_date](#brk_client.BrkClient.index_to_date)
    * [date\_to\_index](#brk_client.BrkClient.date_to_index)
    * [get\_api](#brk_client.BrkClient.get_api)
    * [get\_address](#brk_client.BrkClient.get_address)
    * [get\_address\_txs](#brk_client.BrkClient.get_address_txs)
    * [get\_address\_confirmed\_txs](#brk_client.BrkClient.get_address_confirmed_txs)
    * [get\_address\_mempool\_txs](#brk_client.BrkClient.get_address_mempool_txs)
    * [get\_address\_utxos](#brk_client.BrkClient.get_address_utxos)
    * [get\_block\_by\_height](#brk_client.BrkClient.get_block_by_height)
    * [get\_block](#brk_client.BrkClient.get_block)
    * [get\_block\_header](#brk_client.BrkClient.get_block_header)
    * [get\_block\_raw](#brk_client.BrkClient.get_block_raw)
    * [get\_block\_status](#brk_client.BrkClient.get_block_status)
    * [get\_block\_txid](#brk_client.BrkClient.get_block_txid)
    * [get\_block\_txids](#brk_client.BrkClient.get_block_txids)
    * [get\_block\_txs](#brk_client.BrkClient.get_block_txs)
    * [get\_block\_txs\_from\_index](#brk_client.BrkClient.get_block_txs_from_index)
    * [get\_blocks](#brk_client.BrkClient.get_blocks)
    * [get\_block\_tip\_hash](#brk_client.BrkClient.get_block_tip_hash)
    * [get\_block\_tip\_height](#brk_client.BrkClient.get_block_tip_height)
    * [get\_blocks\_from\_height](#brk_client.BrkClient.get_blocks_from_height)
    * [get\_mempool](#brk_client.BrkClient.get_mempool)
    * [get\_live\_price](#brk_client.BrkClient.get_live_price)
    * [get\_mempool\_recent](#brk_client.BrkClient.get_mempool_recent)
    * [get\_mempool\_txids](#brk_client.BrkClient.get_mempool_txids)
    * [get\_series\_tree](#brk_client.BrkClient.get_series_tree)
    * [get\_series\_bulk](#brk_client.BrkClient.get_series_bulk)
    * [get\_cost\_basis\_cohorts](#brk_client.BrkClient.get_cost_basis_cohorts)
    * [get\_cost\_basis\_dates](#brk_client.BrkClient.get_cost_basis_dates)
    * [get\_cost\_basis](#brk_client.BrkClient.get_cost_basis)
    * [get\_series\_count](#brk_client.BrkClient.get_series_count)
    * [get\_indexes](#brk_client.BrkClient.get_indexes)
    * [list\_series](#brk_client.BrkClient.list_series)
    * [search\_series](#brk_client.BrkClient.search_series)
    * [get\_series\_info](#brk_client.BrkClient.get_series_info)
    * [get\_series](#brk_client.BrkClient.get_series)
    * [get\_series\_data](#brk_client.BrkClient.get_series_data)
    * [get\_series\_latest](#brk_client.BrkClient.get_series_latest)
    * [get\_series\_len](#brk_client.BrkClient.get_series_len)
    * [get\_series\_version](#brk_client.BrkClient.get_series_version)
    * [get\_disk\_usage](#brk_client.BrkClient.get_disk_usage)
    * [get\_sync\_status](#brk_client.BrkClient.get_sync_status)
    * [get\_tx](#brk_client.BrkClient.get_tx)
    * [get\_tx\_hex](#brk_client.BrkClient.get_tx_hex)
    * [get\_tx\_merkle\_proof](#brk_client.BrkClient.get_tx_merkle_proof)
    * [get\_tx\_merkleblock\_proof](#brk_client.BrkClient.get_tx_merkleblock_proof)
    * [get\_tx\_outspend](#brk_client.BrkClient.get_tx_outspend)
    * [get\_tx\_outspends](#brk_client.BrkClient.get_tx_outspends)
    * [get\_tx\_raw](#brk_client.BrkClient.get_tx_raw)
    * [get\_tx\_status](#brk_client.BrkClient.get_tx_status)
    * [get\_block\_v1](#brk_client.BrkClient.get_block_v1)
    * [get\_blocks\_v1](#brk_client.BrkClient.get_blocks_v1)
    * [get\_blocks\_v1\_from\_height](#brk_client.BrkClient.get_blocks_v1_from_height)
    * [get\_cpfp](#brk_client.BrkClient.get_cpfp)
    * [get\_difficulty\_adjustment](#brk_client.BrkClient.get_difficulty_adjustment)
    * [get\_mempool\_blocks](#brk_client.BrkClient.get_mempool_blocks)
    * [get\_precise\_fees](#brk_client.BrkClient.get_precise_fees)
    * [get\_recommended\_fees](#brk_client.BrkClient.get_recommended_fees)
    * [get\_historical\_price](#brk_client.BrkClient.get_historical_price)
    * [get\_block\_fee\_rates](#brk_client.BrkClient.get_block_fee_rates)
    * [get\_block\_fees](#brk_client.BrkClient.get_block_fees)
    * [get\_block\_rewards](#brk_client.BrkClient.get_block_rewards)
    * [get\_block\_sizes\_weights](#brk_client.BrkClient.get_block_sizes_weights)
    * [get\_block\_by\_timestamp](#brk_client.BrkClient.get_block_by_timestamp)
    * [get\_difficulty\_adjustments](#brk_client.BrkClient.get_difficulty_adjustments)
    * [get\_difficulty\_adjustments\_by\_period](#brk_client.BrkClient.get_difficulty_adjustments_by_period)
    * [get\_hashrate](#brk_client.BrkClient.get_hashrate)
    * [get\_pools\_hashrate](#brk_client.BrkClient.get_pools_hashrate)
    * [get\_pools\_hashrate\_by\_period](#brk_client.BrkClient.get_pools_hashrate_by_period)
    * [get\_hashrate\_by\_period](#brk_client.BrkClient.get_hashrate_by_period)
    * [get\_pool](#brk_client.BrkClient.get_pool)
    * [get\_pool\_blocks](#brk_client.BrkClient.get_pool_blocks)
    * [get\_pool\_blocks\_from](#brk_client.BrkClient.get_pool_blocks_from)
    * [get\_pool\_hashrate](#brk_client.BrkClient.get_pool_hashrate)
    * [get\_pools](#brk_client.BrkClient.get_pools)
    * [get\_pool\_stats](#brk_client.BrkClient.get_pool_stats)
    * [get\_reward\_stats](#brk_client.BrkClient.get_reward_stats)
    * [get\_prices](#brk_client.BrkClient.get_prices)
    * [get\_transaction\_times](#brk_client.BrkClient.get_transaction_times)
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

<a id="brk_client.BrkClient"></a>

## BrkClient Objects

```python
class BrkClient(BrkClientBase)
```

Main BRK client with series tree and API methods.

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

<a id="brk_client.BrkClient.CLASS_NAMES"></a>

#### CLASS\_NAMES

<a id="brk_client.BrkClient.SPENDABLE_TYPE_NAMES"></a>

#### SPENDABLE\_TYPE\_NAMES

<a id="brk_client.BrkClient.AGE_RANGE_NAMES"></a>

#### AGE\_RANGE\_NAMES

<a id="brk_client.BrkClient.UNDER_AGE_NAMES"></a>

#### UNDER\_AGE\_NAMES

<a id="brk_client.BrkClient.OVER_AGE_NAMES"></a>

#### OVER\_AGE\_NAMES

<a id="brk_client.BrkClient.AMOUNT_RANGE_NAMES"></a>

#### AMOUNT\_RANGE\_NAMES

<a id="brk_client.BrkClient.OVER_AMOUNT_NAMES"></a>

#### OVER\_AMOUNT\_NAMES

<a id="brk_client.BrkClient.UNDER_AMOUNT_NAMES"></a>

#### UNDER\_AMOUNT\_NAMES

<a id="brk_client.BrkClient.PROFITABILITY_RANGE_NAMES"></a>

#### PROFITABILITY\_RANGE\_NAMES

<a id="brk_client.BrkClient.PROFIT_NAMES"></a>

#### PROFIT\_NAMES

<a id="brk_client.BrkClient.LOSS_NAMES"></a>

#### LOSS\_NAMES

<a id="brk_client.BrkClient.__init__"></a>

#### \_\_init\_\_

```python
def __init__(base_url: str = 'http://localhost:3000', timeout: float = 30.0)
```

<a id="brk_client.BrkClient.series_endpoint"></a>

#### series\_endpoint

```python
def series_endpoint(series: str, index: Index) -> SeriesEndpoint[Any]
```

Create a dynamic series endpoint builder for any series/index combination.

Use this for programmatic access when the series name is determined at runtime.
For type-safe access, use the `series` tree instead.

<a id="brk_client.BrkClient.index_to_date"></a>

#### index\_to\_date

```python
def index_to_date(index: Index, i: int) -> Union[date, datetime]
```

Convert an index value to a date/datetime for date-based indexes.

<a id="brk_client.BrkClient.date_to_index"></a>

#### date\_to\_index

```python
def date_to_index(index: Index, d: Union[date, datetime]) -> int
```

Convert a date/datetime to an index value for date-based indexes.

<a id="brk_client.BrkClient.get_api"></a>

#### get\_api

```python
def get_api() -> str
```

Compact OpenAPI specification.

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

<a id="brk_client.BrkClient.get_address"></a>

#### get\_address

```python
def get_address(address: Addr) -> AddrStats
```

Address information.

Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*

Endpoint: `GET /api/address/{address}`

<a id="brk_client.BrkClient.get_address_txs"></a>

#### get\_address\_txs

```python
def get_address_txs(address: Addr,
                    after_txid: Optional[Txid] = None) -> List[Transaction]
```

Address transactions.

Get transaction history for an address, sorted with newest first. Returns up to 50 mempool transactions plus the first 25 confirmed transactions. Use ?after_txid=<txid> for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*

Endpoint: `GET /api/address/{address}/txs`

<a id="brk_client.BrkClient.get_address_confirmed_txs"></a>

#### get\_address\_confirmed\_txs

```python
def get_address_confirmed_txs(address: Addr,
                              after_txid: Optional[Txid] = None
                              ) -> List[Transaction]
```

Address confirmed transactions.

Get confirmed transactions for an address, 25 per page. Use ?after_txid=<txid> for pagination.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*

Endpoint: `GET /api/address/{address}/txs/chain`

<a id="brk_client.BrkClient.get_address_mempool_txs"></a>

#### get\_address\_mempool\_txs

```python
def get_address_mempool_txs(address: Addr) -> List[Txid]
```

Address mempool transactions.

Get unconfirmed transaction IDs for an address from the mempool (up to 50).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*

Endpoint: `GET /api/address/{address}/txs/mempool`

<a id="brk_client.BrkClient.get_address_utxos"></a>

#### get\_address\_utxos

```python
def get_address_utxos(address: Addr) -> List[Utxo]
```

Address UTXOs.

Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*

Endpoint: `GET /api/address/{address}/utxo`

<a id="brk_client.BrkClient.get_block_by_height"></a>

#### get\_block\_by\_height

```python
def get_block_by_height(height: Height) -> str
```

Block hash by height.

Retrieve the block hash at a given height. Returns the hash as plain text.

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

<a id="brk_client.BrkClient.get_block_header"></a>

#### get\_block\_header

```python
def get_block_header(hash: BlockHash) -> str
```

Block header.

Returns the hex-encoded block header.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-header)*

Endpoint: `GET /api/block/{hash}/header`

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
def get_block_txid(hash: BlockHash, index: TxIndex) -> str
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
def get_block_txs(hash: BlockHash) -> List[Transaction]
```

Block transactions.

Retrieve transactions in a block by block hash. Returns up to 25 transactions starting from index 0.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*

Endpoint: `GET /api/block/{hash}/txs`

<a id="brk_client.BrkClient.get_block_txs_from_index"></a>

#### get\_block\_txs\_from\_index

```python
def get_block_txs_from_index(hash: BlockHash,
                             start_index: TxIndex) -> List[Transaction]
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

<a id="brk_client.BrkClient.get_block_tip_hash"></a>

#### get\_block\_tip\_hash

```python
def get_block_tip_hash() -> str
```

Block tip hash.

Returns the hash of the last block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-tip-hash)*

Endpoint: `GET /api/blocks/tip/hash`

<a id="brk_client.BrkClient.get_block_tip_height"></a>

#### get\_block\_tip\_height

```python
def get_block_tip_height() -> str
```

Block tip height.

Returns the height of the last block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-tip-height)*

Endpoint: `GET /api/blocks/tip/height`

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

Get current mempool statistics including transaction count, total vsize, total fees, and fee histogram.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool`

<a id="brk_client.BrkClient.get_live_price"></a>

#### get\_live\_price

```python
def get_live_price() -> Dollars
```

Live BTC/USD price.

Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.

Endpoint: `GET /api/mempool/price`

<a id="brk_client.BrkClient.get_mempool_recent"></a>

#### get\_mempool\_recent

```python
def get_mempool_recent() -> List[MempoolRecentTx]
```

Recent mempool transactions.

Get the last 10 transactions to enter the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-recent)*

Endpoint: `GET /api/mempool/recent`

<a id="brk_client.BrkClient.get_mempool_txids"></a>

#### get\_mempool\_txids

```python
def get_mempool_txids() -> List[Txid]
```

Mempool transaction IDs.

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

<a id="brk_client.BrkClient.get_series_tree"></a>

#### get\_series\_tree

```python
def get_series_tree() -> TreeNode
```

Series catalog.

Returns the complete hierarchical catalog of available series organized as a tree structure. Series are grouped by categories and subcategories.

Endpoint: `GET /api/series`

<a id="brk_client.BrkClient.get_series_bulk"></a>

#### get\_series\_bulk

```python
def get_series_bulk(
        series: SeriesList,
        index: Index,
        start: Optional[RangeIndex] = None,
        end: Optional[RangeIndex] = None,
        limit: Optional[Limit] = None,
        format: Optional[Format] = None) -> Union[List[AnySeriesData], str]
```

Bulk series data.

Fetch multiple series in a single request. Supports filtering by index and date range. Returns an array of SeriesData objects. For a single series, use `get_series` instead.

Endpoint: `GET /api/series/bulk`

<a id="brk_client.BrkClient.get_cost_basis_cohorts"></a>

#### get\_cost\_basis\_cohorts

```python
def get_cost_basis_cohorts() -> List[str]
```

Available cost basis cohorts.

List available cohorts for cost basis distribution.

Endpoint: `GET /api/series/cost-basis`

<a id="brk_client.BrkClient.get_cost_basis_dates"></a>

#### get\_cost\_basis\_dates

```python
def get_cost_basis_dates(cohort: Cohort) -> List[Date]
```

Available cost basis dates.

List available dates for a cohort's cost basis distribution.

Endpoint: `GET /api/series/cost-basis/{cohort}/dates`

<a id="brk_client.BrkClient.get_cost_basis"></a>

#### get\_cost\_basis

```python
def get_cost_basis(cohort: Cohort,
                   date: str,
                   bucket: Optional[CostBasisBucket] = None,
                   value: Optional[CostBasisValue] = None) -> dict
```

Cost basis distribution.

Get the cost basis distribution for a cohort on a specific date.

Query params:
- `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100
- `value`: supply (default, in BTC), realized (USD), unrealized (USD)

Endpoint: `GET /api/series/cost-basis/{cohort}/{date}`

<a id="brk_client.BrkClient.get_series_count"></a>

#### get\_series\_count

```python
def get_series_count() -> List[SeriesCount]
```

Series count.

Returns the number of series available per index type.

Endpoint: `GET /api/series/count`

<a id="brk_client.BrkClient.get_indexes"></a>

#### get\_indexes

```python
def get_indexes() -> List[IndexInfo]
```

List available indexes.

Returns all available indexes with their accepted query aliases. Use any alias when querying series.

Endpoint: `GET /api/series/indexes`

<a id="brk_client.BrkClient.list_series"></a>

#### list\_series

```python
def list_series(page: Optional[float] = None,
                per_page: Optional[float] = None) -> PaginatedSeries
```

Series list.

Paginated flat list of all available series names. Use `page` query param for pagination.

Endpoint: `GET /api/series/list`

<a id="brk_client.BrkClient.search_series"></a>

#### search\_series

```python
def search_series(q: SeriesName, limit: Optional[Limit] = None) -> List[str]
```

Search series.

Fuzzy search for series by name. Supports partial matches and typos.

Endpoint: `GET /api/series/search`

<a id="brk_client.BrkClient.get_series_info"></a>

#### get\_series\_info

```python
def get_series_info(series: SeriesName) -> SeriesInfo
```

Get series info.

Returns the supported indexes and value type for the specified series.

Endpoint: `GET /api/series/{series}`

<a id="brk_client.BrkClient.get_series"></a>

#### get\_series

```python
def get_series(series: SeriesName,
               index: Index,
               start: Optional[RangeIndex] = None,
               end: Optional[RangeIndex] = None,
               limit: Optional[Limit] = None,
               format: Optional[Format] = None) -> Union[AnySeriesData, str]
```

Get series data.

Fetch data for a specific series at the given index. Use query parameters to filter by date range and format (json/csv).

Endpoint: `GET /api/series/{series}/{index}`

<a id="brk_client.BrkClient.get_series_data"></a>

#### get\_series\_data

```python
def get_series_data(series: SeriesName,
                    index: Index,
                    start: Optional[RangeIndex] = None,
                    end: Optional[RangeIndex] = None,
                    limit: Optional[Limit] = None,
                    format: Optional[Format] = None) -> Union[List[bool], str]
```

Get raw series data.

Returns just the data array without the SeriesData wrapper. Supports the same range and format parameters as the standard endpoint.

Endpoint: `GET /api/series/{series}/{index}/data`

<a id="brk_client.BrkClient.get_series_latest"></a>

#### get\_series\_latest

```python
def get_series_latest(series: SeriesName, index: Index) -> str
```

Get latest series value.

Returns the single most recent value for a series, unwrapped (not inside a SeriesData object).

Endpoint: `GET /api/series/{series}/{index}/latest`

<a id="brk_client.BrkClient.get_series_len"></a>

#### get\_series\_len

```python
def get_series_len(series: SeriesName, index: Index) -> float
```

Get series data length.

Returns the total number of data points for a series at the given index.

Endpoint: `GET /api/series/{series}/{index}/len`

<a id="brk_client.BrkClient.get_series_version"></a>

#### get\_series\_version

```python
def get_series_version(series: SeriesName, index: Index) -> Version
```

Get series version.

Returns the current version of a series. Changes when the series data is updated.

Endpoint: `GET /api/series/{series}/{index}/version`

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
def get_tx_hex(txid: Txid) -> str
```

Transaction hex.

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*

Endpoint: `GET /api/tx/{txid}/hex`

<a id="brk_client.BrkClient.get_tx_merkle_proof"></a>

#### get\_tx\_merkle\_proof

```python
def get_tx_merkle_proof(txid: Txid) -> MerkleProof
```

Transaction merkle proof.

Get the merkle inclusion proof for a transaction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkle-proof)*

Endpoint: `GET /api/tx/{txid}/merkle-proof`

<a id="brk_client.BrkClient.get_tx_merkleblock_proof"></a>

#### get\_tx\_merkleblock\_proof

```python
def get_tx_merkleblock_proof(txid: Txid) -> str
```

Transaction merkleblock proof.

Get the merkleblock proof for a transaction (BIP37 format, hex encoded).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkleblock-proof)*

Endpoint: `GET /api/tx/{txid}/merkleblock-proof`

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

<a id="brk_client.BrkClient.get_tx_raw"></a>

#### get\_tx\_raw

```python
def get_tx_raw(txid: Txid) -> List[float]
```

Transaction raw.

Returns a transaction as binary data.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-raw)*

Endpoint: `GET /api/tx/{txid}/raw`

<a id="brk_client.BrkClient.get_tx_status"></a>

#### get\_tx\_status

```python
def get_tx_status(txid: Txid) -> TxStatus
```

Transaction status.

Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*

Endpoint: `GET /api/tx/{txid}/status`

<a id="brk_client.BrkClient.get_block_v1"></a>

#### get\_block\_v1

```python
def get_block_v1(hash: BlockHash) -> BlockInfoV1
```

Block (v1).

Returns block details with extras by hash.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-v1)*

Endpoint: `GET /api/v1/block/{hash}`

<a id="brk_client.BrkClient.get_blocks_v1"></a>

#### get\_blocks\_v1

```python
def get_blocks_v1() -> List[BlockInfoV1]
```

Recent blocks with extras.

Retrieve the last 10 blocks with extended data including pool identification and fee statistics.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks-v1)*

Endpoint: `GET /api/v1/blocks`

<a id="brk_client.BrkClient.get_blocks_v1_from_height"></a>

#### get\_blocks\_v1\_from\_height

```python
def get_blocks_v1_from_height(height: Height) -> List[BlockInfoV1]
```

Blocks from height with extras.

Retrieve up to 10 blocks with extended data going backwards from the given height.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks-v1)*

Endpoint: `GET /api/v1/blocks/{height}`

<a id="brk_client.BrkClient.get_cpfp"></a>

#### get\_cpfp

```python
def get_cpfp(txid: Txid) -> CpfpInfo
```

CPFP info.

Returns ancestors and descendants for a CPFP transaction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-children-pay-for-parent)*

Endpoint: `GET /api/v1/cpfp/{txid}`

<a id="brk_client.BrkClient.get_difficulty_adjustment"></a>

#### get\_difficulty\_adjustment

```python
def get_difficulty_adjustment() -> DifficultyAdjustment
```

Difficulty adjustment.

Get current difficulty adjustment progress and estimates.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

<a id="brk_client.BrkClient.get_mempool_blocks"></a>

#### get\_mempool\_blocks

```python
def get_mempool_blocks() -> List[MempoolBlock]
```

Projected mempool blocks.

Get projected blocks from the mempool for fee estimation.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

<a id="brk_client.BrkClient.get_precise_fees"></a>

#### get\_precise\_fees

```python
def get_precise_fees() -> RecommendedFees
```

Precise recommended fees.

Get recommended fee rates with up to 3 decimal places.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees-precise)*

Endpoint: `GET /api/v1/fees/precise`

<a id="brk_client.BrkClient.get_recommended_fees"></a>

#### get\_recommended\_fees

```python
def get_recommended_fees() -> RecommendedFees
```

Recommended fees.

Get recommended fee rates for different confirmation targets.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

<a id="brk_client.BrkClient.get_historical_price"></a>

#### get\_historical\_price

```python
def get_historical_price(
        timestamp: Optional[Timestamp] = None) -> HistoricalPrice
```

Historical price.

Get historical BTC/USD price. Optionally specify a UNIX timestamp to get the price at that time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-historical-price)*

Endpoint: `GET /api/v1/historical-price`

<a id="brk_client.BrkClient.get_block_fee_rates"></a>

#### get\_block\_fee\_rates

```python
def get_block_fee_rates(time_period: TimePeriod) -> List[BlockFeeRatesEntry]
```

Block fee rates.

Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

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

<a id="brk_client.BrkClient.get_pools_hashrate"></a>

#### get\_pools\_hashrate

```python
def get_pools_hashrate() -> List[PoolHashrateEntry]
```

All pools hashrate (all time).

Get hashrate data for all mining pools.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrates)*

Endpoint: `GET /api/v1/mining/hashrate/pools`

<a id="brk_client.BrkClient.get_pools_hashrate_by_period"></a>

#### get\_pools\_hashrate\_by\_period

```python
def get_pools_hashrate_by_period(
        time_period: TimePeriod) -> List[PoolHashrateEntry]
```

All pools hashrate.

Get hashrate data for all mining pools for a time period. Valid periods: 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrates)*

Endpoint: `GET /api/v1/mining/hashrate/pools/{time_period}`

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

<a id="brk_client.BrkClient.get_pool_blocks"></a>

#### get\_pool\_blocks

```python
def get_pool_blocks(slug: PoolSlug) -> List[BlockInfoV1]
```

Mining pool blocks.

Get the 10 most recent blocks mined by a specific pool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-blocks)*

Endpoint: `GET /api/v1/mining/pool/{slug}/blocks`

<a id="brk_client.BrkClient.get_pool_blocks_from"></a>

#### get\_pool\_blocks\_from

```python
def get_pool_blocks_from(slug: PoolSlug, height: Height) -> List[BlockInfoV1]
```

Mining pool blocks from height.

Get 10 blocks mined by a specific pool before (and including) the given height.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-blocks)*

Endpoint: `GET /api/v1/mining/pool/{slug}/blocks/{height}`

<a id="brk_client.BrkClient.get_pool_hashrate"></a>

#### get\_pool\_hashrate

```python
def get_pool_hashrate(slug: PoolSlug) -> List[PoolHashrateEntry]
```

Mining pool hashrate.

Get hashrate history for a specific mining pool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrate)*

Endpoint: `GET /api/v1/mining/pool/{slug}/hashrate`

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

<a id="brk_client.BrkClient.get_prices"></a>

#### get\_prices

```python
def get_prices() -> Prices
```

Current BTC price.

Returns bitcoin latest price (on-chain derived, USD only).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-price)*

Endpoint: `GET /api/v1/prices`

<a id="brk_client.BrkClient.get_transaction_times"></a>

#### get\_transaction\_times

```python
def get_transaction_times() -> List[float]
```

Transaction first-seen times.

Returns timestamps when transactions were first seen in the mempool. Returns 0 for mined or unknown transactions.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-times)*

Endpoint: `GET /api/v1/transaction-times`

<a id="brk_client.BrkClient.validate_address"></a>

#### validate\_address

```python
def validate_address(address: str) -> AddrValidation
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
def get_openapi() -> str
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

