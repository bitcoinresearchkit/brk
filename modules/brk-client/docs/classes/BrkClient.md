[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClient

# Class: BrkClient

Defined in: [Developer/brk/modules/brk-client/index.js:6565](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L6565)

Main BRK client with series tree and API methods

## Extends

- `BrkClientBase`

## Constructors

### Constructor

> **new BrkClient**(`options`): `BrkClient`

Defined in: [Developer/brk/modules/brk-client/index.js:7744](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L7744)

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

Defined in: [Developer/brk/modules/brk-client/index.js:1526](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1526)

#### Inherited from

`BrkClientBase._cache`

***

### \_cachePromise

> **\_cachePromise**: `Promise`\<`Cache` \| `null`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1524](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1524)

#### Inherited from

`BrkClientBase._cachePromise`

***

### series

> **series**: [`SeriesTree`](../interfaces/SeriesTree.md)

Defined in: [Developer/brk/modules/brk-client/index.js:7747](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L7747)

## Methods

### \_fetchSeriesData()

> **\_fetchSeriesData**\<`T`\>(`path`, `onUpdate?`): `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1614](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1614)

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

Defined in: [Developer/brk/modules/brk-client/index.js:7736](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L7736)

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

Defined in: [Developer/brk/modules/brk-client/index.js:1534](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1534)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9452](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9452)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9490](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9490)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9510](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9510)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9469](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9469)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9526](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9526)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9436](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9436)

Compact OpenAPI specification

Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

Endpoint: `GET /api.json`

#### Returns

`Promise`\<`any`\>

***

### getBlock()

> **getBlock**(`hash`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9558](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9558)

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

> **getBlockByHeight**(`height`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9542](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9542)

Block hash by height

Retrieve the block hash at a given height. Returns the hash as plain text.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*

Endpoint: `GET /api/block-height/{height}`

#### Parameters

##### height

`number`

#### Returns

`Promise`\<`any`\>

***

### getBlockByTimestamp()

> **getBlockByTimestamp**(`timestamp`): `Promise`\<[`BlockTimestamp`](../interfaces/BlockTimestamp.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10431](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10431)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10367](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10367)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10383](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10383)

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

### getBlockHeader()

> **getBlockHeader**(`hash`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9574](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9574)

Block header

Returns the hex-encoded block header.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-header)*

Endpoint: `GET /api/block/{hash}/header`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<`any`\>

***

### getBlockRaw()

> **getBlockRaw**(`hash`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9590](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9590)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10399](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10399)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9686](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9686)

Recent blocks

Retrieve the last 10 blocks. Returns block metadata for each block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

Endpoint: `GET /api/blocks`

#### Returns

`Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

***

### getBlocksFromHeight()

> **getBlocksFromHeight**(`height`): `Promise`\<[`BlockInfo`](../interfaces/BlockInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9730](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9730)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10415](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10415)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9606](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9606)

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

### getBlocksV1()

> **getBlocksV1**(): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10243](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10243)

Recent blocks with extras

Retrieve the last 10 blocks with extended data including pool identification and fee statistics.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks-v1)*

Endpoint: `GET /api/v1/blocks`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getBlocksV1FromHeight()

> **getBlocksV1FromHeight**(`height`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10259](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10259)

Blocks from height with extras

Retrieve up to 10 blocks with extended data going backwards from the given height.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks-v1)*

Endpoint: `GET /api/v1/blocks/{height}`

#### Parameters

##### height

`number`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getBlockTipHash()

> **getBlockTipHash**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9700](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9700)

Block tip hash

Returns the hash of the last block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-tip-hash)*

Endpoint: `GET /api/blocks/tip/hash`

#### Returns

`Promise`\<`any`\>

***

### getBlockTipHeight()

> **getBlockTipHeight**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9714](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9714)

Block tip height

Returns the height of the last block.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-tip-height)*

Endpoint: `GET /api/blocks/tip/height`

#### Returns

`Promise`\<`any`\>

***

### getBlockTxid()

> **getBlockTxid**(`hash`, `index`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9623](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9623)

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

`Promise`\<`any`\>

***

### getBlockTxids()

> **getBlockTxids**(`hash`): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9639](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9639)

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

> **getBlockTxs**(`hash`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9655](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9655)

Block transactions

Retrieve transactions in a block by block hash. Returns up to 25 transactions starting from index 0.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*

Endpoint: `GET /api/block/{hash}/txs`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

***

### getBlockTxsFromIndex()

> **getBlockTxsFromIndex**(`hash`, `start_index`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9672](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9672)

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

### getBlockV1()

> **getBlockV1**(`hash`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10229](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10229)

Block (v1)

Returns block details with extras by hash.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-v1)*

Endpoint: `GET /api/v1/block/{hash}`

#### Parameters

##### hash

`string`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)\>

***

### getCostBasis()

> **getCostBasis**(`cohort`, `date`, `bucket?`, `value?`): `Promise`\<`Object`\>

Defined in: [Developer/brk/modules/brk-client/index.js:9874](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9874)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9839](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9839)

Available cost basis cohorts

List available cohorts for cost basis distribution.

Endpoint: `GET /api/series/cost-basis`

#### Returns

`Promise`\<`string`[]\>

***

### getCostBasisDates()

> **getCostBasisDates**(`cohort`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9853](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9853)

Available cost basis dates

List available dates for a cohort's cost basis distribution.

Endpoint: `GET /api/series/cost-basis/{cohort}/dates`

#### Parameters

##### cohort

`string`

#### Returns

`Promise`\<`number`[]\>

***

### getCpfp()

> **getCpfp**(`txid`): `Promise`\<[`CpfpInfo`](../interfaces/CpfpInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10275](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10275)

CPFP info

Returns ancestors and descendants for a CPFP transaction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-children-pay-for-parent)*

Endpoint: `GET /api/v1/cpfp/{txid}`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`CpfpInfo`](../interfaces/CpfpInfo.md)\>

***

### getDifficultyAdjustment()

> **getDifficultyAdjustment**(): `Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10289](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10289)

Difficulty adjustment

Get current difficulty adjustment progress and estimates.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

Endpoint: `GET /api/v1/difficulty-adjustment`

#### Returns

`Promise`\<[`DifficultyAdjustment`](../interfaces/DifficultyAdjustment.md)\>

***

### getDifficultyAdjustments()

> **getDifficultyAdjustments**(): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10445](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10445)

Difficulty adjustments (all time)

Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

Endpoint: `GET /api/v1/mining/difficulty-adjustments`

#### Returns

`Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

***

### getDifficultyAdjustmentsByPeriod()

> **getDifficultyAdjustmentsByPeriod**(`time_period`): `Promise`\<[`DifficultyAdjustmentEntry`](../interfaces/DifficultyAdjustmentEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10461](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10461)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10072](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10072)

Disk usage

Returns the disk space used by BRK and Bitcoin data.

Endpoint: `GET /api/server/disk`

#### Returns

`Promise`\<[`DiskUsage`](../interfaces/DiskUsage.md)\>

***

### getHashrate()

> **getHashrate**(): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10475](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10475)

Network hashrate (all time)

Get network hashrate and difficulty data for all time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

Endpoint: `GET /api/v1/mining/hashrate`

#### Returns

`Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

***

### getHashrateByPeriod()

> **getHashrateByPeriod**(`time_period`): `Promise`\<[`HashrateSummary`](../interfaces/HashrateSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10521](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10521)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10688](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10688)

Health check

Returns the health status of the API server, including uptime information.

Endpoint: `GET /health`

#### Returns

`Promise`\<[`Health`](../interfaces/Health.md)\>

***

### getHistoricalPrice()

> **getHistoricalPrice**(`timestamp?`): `Promise`\<[`HistoricalPrice`](../interfaces/HistoricalPrice.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10347](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10347)

Historical price

Get historical BTC/USD price. Optionally specify a UNIX timestamp to get the price at that time.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-historical-price)*

Endpoint: `GET /api/v1/historical-price`

#### Parameters

##### timestamp?

`number`

#### Returns

`Promise`\<[`HistoricalPrice`](../interfaces/HistoricalPrice.md)\>

***

### getIndexes()

> **getIndexes**(): `Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9903](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9903)

List available indexes

Returns all available indexes with their accepted query aliases. Use any alias when querying series.

Endpoint: `GET /api/series/indexes`

#### Returns

`Promise`\<[`IndexInfo`](../interfaces/IndexInfo.md)[]\>

***

### getJson()

> **getJson**\<`T`\>(`path`, `onUpdate?`): `Promise`\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1548](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1548)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9756](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9756)

Live BTC/USD price

Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.

Endpoint: `GET /api/mempool/price`

#### Returns

`Promise`\<`number`\>

***

### getMempool()

> **getMempool**(): `Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:9744](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9744)

Mempool statistics

Get current mempool statistics including transaction count, total vsize, total fees, and fee histogram.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

Endpoint: `GET /api/mempool`

#### Returns

`Promise`\<[`MempoolInfo`](../interfaces/MempoolInfo.md)\>

***

### getMempoolBlocks()

> **getMempoolBlocks**(): `Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10303](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10303)

Projected mempool blocks

Get projected blocks from the mempool for fee estimation.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

Endpoint: `GET /api/v1/fees/mempool-blocks`

#### Returns

`Promise`\<[`MempoolBlock`](../interfaces/MempoolBlock.md)[]\>

***

### getMempoolRecent()

> **getMempoolRecent**(): `Promise`\<[`MempoolRecentTx`](../interfaces/MempoolRecentTx.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9770](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9770)

Recent mempool transactions

Get the last 10 transactions to enter the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-recent)*

Endpoint: `GET /api/mempool/recent`

#### Returns

`Promise`\<[`MempoolRecentTx`](../interfaces/MempoolRecentTx.md)[]\>

***

### getMempoolTxids()

> **getMempoolTxids**(): `Promise`\<`string`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:9784](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9784)

Mempool transaction IDs

Get all transaction IDs currently in the mempool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

Endpoint: `GET /api/mempool/txids`

#### Returns

`Promise`\<`string`[]\>

***

### getOpenapi()

> **getOpenapi**(): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10700](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10700)

OpenAPI specification

Full OpenAPI 3.1 specification for this API.

Endpoint: `GET /openapi.json`

#### Returns

`Promise`\<`any`\>

***

### getPool()

> **getPool**(`slug`): `Promise`\<[`PoolDetail`](../interfaces/PoolDetail.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10537](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10537)

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

### getPoolBlocks()

> **getPoolBlocks**(`slug`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10553](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10553)

Mining pool blocks

Get the 10 most recent blocks mined by a specific pool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-blocks)*

Endpoint: `GET /api/v1/mining/pool/{slug}/blocks`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getPoolBlocksFrom()

> **getPoolBlocksFrom**(`slug`, `height`): `Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10570](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10570)

Mining pool blocks from height

Get 10 blocks mined by a specific pool before (and including) the given height.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-blocks)*

Endpoint: `GET /api/v1/mining/pool/{slug}/blocks/{height}`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

##### height

`number`

#### Returns

`Promise`\<[`BlockInfoV1`](../interfaces/BlockInfoV1.md)[]\>

***

### getPoolHashrate()

> **getPoolHashrate**(`slug`): `Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10586](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10586)

Mining pool hashrate

Get hashrate history for a specific mining pool.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrate)*

Endpoint: `GET /api/v1/mining/pool/{slug}/hashrate`

#### Parameters

##### slug

[`PoolSlug`](../type-aliases/PoolSlug.md)

#### Returns

`Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

***

### getPools()

> **getPools**(): `Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10600](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10600)

List all mining pools

Get list of all known mining pools with their identifiers.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

Endpoint: `GET /api/v1/mining/pools`

#### Returns

`Promise`\<[`PoolInfo`](../interfaces/PoolInfo.md)[]\>

***

### getPoolsHashrate()

> **getPoolsHashrate**(): `Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10489](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10489)

All pools hashrate (all time)

Get hashrate data for all mining pools.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrates)*

Endpoint: `GET /api/v1/mining/hashrate/pools`

#### Returns

`Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

***

### getPoolsHashrateByPeriod()

> **getPoolsHashrateByPeriod**(`time_period`): `Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10505](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10505)

All pools hashrate

Get hashrate data for all mining pools for a time period. Valid periods: 1m, 3m, 6m, 1y, 2y, 3y

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool-hashrates)*

Endpoint: `GET /api/v1/mining/hashrate/pools/{time_period}`

#### Parameters

##### time\_period

[`TimePeriod`](../type-aliases/TimePeriod.md)

#### Returns

`Promise`\<[`PoolHashrateEntry`](../interfaces/PoolHashrateEntry.md)[]\>

***

### getPoolStats()

> **getPoolStats**(`time_period`): `Promise`\<[`PoolsSummary`](../interfaces/PoolsSummary.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10616](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10616)

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

### getPreciseFees()

> **getPreciseFees**(): `Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10317](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10317)

Precise recommended fees

Get recommended fee rates with up to 3 decimal places.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees-precise)*

Endpoint: `GET /api/v1/fees/precise`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getPrices()

> **getPrices**(): `Promise`\<[`Prices`](../interfaces/Prices.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10646](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10646)

Current BTC price

Returns bitcoin latest price (on-chain derived, USD only).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-price)*

Endpoint: `GET /api/v1/prices`

#### Returns

`Promise`\<[`Prices`](../interfaces/Prices.md)\>

***

### getRecommendedFees()

> **getRecommendedFees**(): `Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10331](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10331)

Recommended fees

Get recommended fee rates for different confirmation targets.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

Endpoint: `GET /api/v1/fees/recommended`

#### Returns

`Promise`\<[`RecommendedFees`](../interfaces/RecommendedFees.md)\>

***

### getRewardStats()

> **getRewardStats**(`block_count`): `Promise`\<[`RewardStats`](../interfaces/RewardStats.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10632](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10632)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9976](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9976)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9815](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9815)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9891](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9891)

Series count

Returns the number of series available per index type.

Endpoint: `GET /api/series/count`

#### Returns

`Promise`\<[`SeriesCount`](../interfaces/SeriesCount.md)[]\>

***

### getSeriesData()

> **getSeriesData**(`series`, `index`, `start?`, `end?`, `limit?`, `format?`): `Promise`\<`string` \| `boolean`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10005](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10005)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9957](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9957)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10030](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10030)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10045](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10045)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9796](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9796)

Series catalog

Returns the complete hierarchical catalog of available series organized as a tree structure. Series are grouped by categories and subcategories.

Endpoint: `GET /api/series`

#### Returns

`Promise`\<[`TreeNode`](../type-aliases/TreeNode.md)\>

***

### getSeriesVersion()

> **getSeriesVersion**(`series`, `index`): `Promise`\<`number`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10060](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10060)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10084](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10084)

Sync status

Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

Endpoint: `GET /api/server/sync`

#### Returns

`Promise`\<[`SyncStatus`](../interfaces/SyncStatus.md)\>

***

### getText()

> **getText**(`path`): `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1602](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L1602)

Make a GET request and return raw text (for CSV responses)

#### Parameters

##### path

`string`

#### Returns

`Promise`\<`string`\>

#### Inherited from

`BrkClientBase.getText`

***

### getTransactionTimes()

> **getTransactionTimes**(): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10660](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10660)

Transaction first-seen times

Returns timestamps when transactions were first seen in the mempool. Returns 0 for mined or unknown transactions.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-times)*

Endpoint: `GET /api/v1/transaction-times`

#### Returns

`Promise`\<`number`[]\>

***

### getTx()

> **getTx**(`txid`): `Promise`\<[`Transaction`](../interfaces/Transaction.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10100](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10100)

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

> **getTxHex**(`txid`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10116](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10116)

Transaction hex

Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*

Endpoint: `GET /api/tx/{txid}/hex`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<`any`\>

***

### getTxMerkleblockProof()

> **getTxMerkleblockProof**(`txid`): `Promise`\<`any`\>

Defined in: [Developer/brk/modules/brk-client/index.js:10148](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10148)

Transaction merkleblock proof

Get the merkleblock proof for a transaction (BIP37 format, hex encoded).

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkleblock-proof)*

Endpoint: `GET /api/tx/{txid}/merkleblock-proof`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<`any`\>

***

### getTxMerkleProof()

> **getTxMerkleProof**(`txid`): `Promise`\<[`MerkleProof`](../interfaces/MerkleProof.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10132](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10132)

Transaction merkle proof

Get the merkle inclusion proof for a transaction.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkle-proof)*

Endpoint: `GET /api/tx/{txid}/merkle-proof`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<[`MerkleProof`](../interfaces/MerkleProof.md)\>

***

### getTxOutspend()

> **getTxOutspend**(`txid`, `vout`): `Promise`\<[`TxOutspend`](../interfaces/TxOutspend.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10165](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10165)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10181](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10181)

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

### getTxRaw()

> **getTxRaw**(`txid`): `Promise`\<`number`[]\>

Defined in: [Developer/brk/modules/brk-client/index.js:10197](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10197)

Transaction raw

Returns a transaction as binary data.

*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-raw)*

Endpoint: `GET /api/tx/{txid}/raw`

#### Parameters

##### txid

`string`

#### Returns

`Promise`\<`number`[]\>

***

### getTxStatus()

> **getTxStatus**(`txid`): `Promise`\<[`TxStatus`](../interfaces/TxStatus.md)\>

Defined in: [Developer/brk/modules/brk-client/index.js:10213](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10213)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10712](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10712)

API version

Returns the current version of the API server

Endpoint: `GET /version`

#### Returns

`Promise`\<`string`\>

***

### indexToDate()

> **indexToDate**(`index`, `i`): `Date`

Defined in: [Developer/brk/modules/brk-client/index.js:7726](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L7726)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9918](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9918)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9938](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9938)

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

Defined in: [Developer/brk/modules/brk-client/index.js:9424](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L9424)

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

Defined in: [Developer/brk/modules/brk-client/index.js:10676](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L10676)

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
