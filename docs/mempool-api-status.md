# Mempool.space API Compatibility - Implementation Status

Plan file: `/Users/k/.claude/plans/smooth-weaving-crayon.md`

## Completed Endpoints

| Endpoint | Path | Notes |
|----------|------|-------|
| GET Block | `/api/block/{hash}` | |
| GET Block Height | `/api/block-height/{height}` | Returns plain text hash |
| GET Block Status | `/api/block/{hash}/status` | |
| GET Block Txids | `/api/block/{hash}/txids` | |
| GET Blocks | `/api/blocks[/:start_height]` | Last 10 blocks |
| GET Transaction | `/api/tx/{txid}` | |
| GET Tx Status | `/api/tx/{txid}/status` | |
| GET Tx Hex | `/api/tx/{txid}/hex` | Returns plain text |
| GET Address | `/api/address/{address}` | |
| GET Address Txs | `/api/address/{address}/txs` | |
| GET Address UTXOs | `/api/address/{address}/utxo` | |
| GET Mempool Info | `/api/mempool/info` | |
| GET Mempool Txids | `/api/mempool/txids` | |
| GET Recommended Fees | `/api/v1/fees/recommended` | Basic impl, needs optimization |

## Remaining Endpoints

### Mempool/Fees (4)

| # | Endpoint | Path | Dependencies | Priority |
|---|----------|------|--------------|----------|
| 1 | Optimize projected blocks | - | CPFP/ancestor scores | HIGH |
| 2 | GET Mempool Blocks | `/api/v1/fees/mempool-blocks` | #1 | HIGH |
| 3 | GET Mempool Recent | `/api/mempool/recent` | | MED |
| 4 | GET RBF Replacements | `/api/v1/replacements` | RBF tracking in brk_monitor | LOW |

### Blocks (4)

| # | Endpoint | Path | Dependencies | Priority |
|---|----------|------|--------------|----------|
| 5 | GET Block Txs | `/api/block/{hash}/txs[/:start_index]` | | MED |
| 6 | GET Block Txid at Index | `/api/block/{hash}/txid/{index}` | | LOW |
| 7 | GET Block Raw | `/api/block/{hash}/raw` | brk_reader | LOW |
| 8 | GET Block by Timestamp | `/api/v1/mining/blocks/timestamp/{timestamp}` | Binary search | LOW |

### Addresses (3)

| # | Endpoint | Path | Dependencies | Priority |
|---|----------|------|--------------|----------|
| 9 | GET Address Txs Chain | `/api/address/{address}/txs/chain[/:after_txid]` | | MED |
| 10 | GET Address Txs Mempool | `/api/address/{address}/txs/mempool` | brk_monitor | MED |
| 11 | GET Validate Address | `/api/v1/validate-address/{address}` | | LOW |

### Transactions (4)

| # | Endpoint | Path | Dependencies | Priority |
|---|----------|------|--------------|----------|
| 12 | GET Tx Outspend | `/api/tx/{txid}/outspend/{vout}` | #27 txoutindex_to_txinindex | HIGH |
| 13 | GET Tx Outspends | `/api/tx/{txid}/outspends` | #27 | HIGH |
| 14 | GET Tx Merkle Proof | `/api/tx/{txid}/merkle-proof` | | LOW |
| 15 | POST Tx Broadcast | `/api/tx` | brk_rpc | MED |

### General (1)

| # | Endpoint | Path | Dependencies | Priority |
|---|----------|------|--------------|----------|
| 16 | GET Difficulty Adjustment | `/api/v1/difficulty-adjustment` | | MED |

### Mining (9)

| # | Endpoint | Path | Dependencies | Priority |
|---|----------|------|--------------|----------|
| 17 | GET Mining Pools | `/api/v1/mining/pools[/:timePeriod]` | #28 pool identification | LOW |
| 18 | GET Mining Pool | `/api/v1/mining/pool/{slug}` | #28 | LOW |
| 19 | GET Hashrate | `/api/v1/mining/hashrate[/:timePeriod]` | | MED |
| 20 | GET Difficulty Adjustments | `/api/v1/mining/difficulty-adjustments[/:interval]` | | LOW |
| 21 | GET Reward Stats | `/api/v1/mining/reward-stats/{blockCount}` | | LOW |
| 22 | GET Block Fees | `/api/v1/mining/blocks/fees/{timePeriod}` | | LOW |
| 23 | GET Block Rewards | `/api/v1/mining/blocks/rewards/{timePeriod}` | | LOW |
| 24 | GET Block Fee Rates | `/api/v1/mining/blocks/fee-rates/{timePeriod}` | | LOW |
| 25 | GET Block Sizes/Weights | `/api/v1/mining/blocks/sizes-weights/{timePeriod}` | | LOW |

### Infrastructure (3)

| # | Task | Location | Priority |
|---|------|----------|----------|
| 26 | Index txindex_to_sigop_cost | brk_indexer | MED |
| 27 | Add txoutindex_to_txinindex mapping | brk_computer/stateful | HIGH |
| 28 | Pool identification from coinbase | brk_computer | LOW |

## Priority Order

### Phase 1: Core Functionality (HIGH)
1. **#27** Add txoutindex_to_txinindex mapping (enables outspend lookups)
2. **#12** GET Tx Outspend
3. **#13** GET Tx Outspends
4. **#1** Optimize projected blocks (CPFP/ancestor scores)
5. **#2** GET Mempool Blocks

### Phase 2: Essential Features (MED)
6. **#15** POST Tx Broadcast
7. **#16** GET Difficulty Adjustment
8. **#5** GET Block Txs (paginated)
9. **#9** GET Address Txs Chain
10. **#10** GET Address Txs Mempool
11. **#19** GET Hashrate
12. **#26** Index txindex_to_sigop_cost
13. **#3** GET Mempool Recent

### Phase 3: Nice to Have (LOW)
14. **#6** GET Block Txid at Index
15. **#7** GET Block Raw
16. **#8** GET Block by Timestamp
17. **#11** GET Validate Address
18. **#14** GET Tx Merkle Proof
19. **#4** GET RBF Replacements
20. **#20** GET Difficulty Adjustments
21. **#21** GET Reward Stats
22. **#22-25** Mining block statistics
23. **#17-18** Mining pools (requires #28)
24. **#28** Pool identification

## Design Documents

- Mempool projected blocks: `crates/brk_monitor/src/mempool/DESIGN.md`

## Skipped Endpoints

| Endpoint | Reason |
|----------|--------|
| GET Price | `/api/v1/prices` | External data source needed |
| GET Historical Price | `/api/v1/historical-price` | External data source needed |
| GET Full-RBF Replacements | `/api/v1/fullrbf/replacements` | Low priority |
| Lightning endpoints | Requires separate Lightning indexing |
| Accelerator endpoints | mempool.space-specific paid service |
