[**brk-client**](../README.md)

***

[brk-client](../globals.md) / PoolDetail

# Interface: PoolDetail

Defined in: [Developer/brk/modules/brk-client/index.js:883](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L883)

## Properties

### blockCount

> **blockCount**: [`PoolBlockCounts`](PoolBlockCounts.md)

Defined in: [Developer/brk/modules/brk-client/index.js:885](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L885)

Block counts for different time periods

***

### blockShare

> **blockShare**: [`PoolBlockShares`](PoolBlockShares.md)

Defined in: [Developer/brk/modules/brk-client/index.js:886](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L886)

Pool's share of total blocks for different time periods

***

### estimatedHashrate

> **estimatedHashrate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:887](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L887)

Estimated hashrate based on blocks mined (H/s)

***

### pool

> **pool**: [`PoolDetailInfo`](PoolDetailInfo.md)

Defined in: [Developer/brk/modules/brk-client/index.js:884](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L884)

Pool information

***

### reportedHashrate?

> `optional` **reportedHashrate?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:888](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L888)

Self-reported hashrate (if available, H/s)

***

### totalReward?

> `optional` **totalReward?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:889](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L889)

Total reward earned by this pool (sats, all time; None for minor pools)
