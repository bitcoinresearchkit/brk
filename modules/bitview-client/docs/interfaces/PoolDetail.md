[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / PoolDetail

# Interface: PoolDetail

Defined in: [Developer/mono/modules/bitview-client/index.js:907](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L907)

## Properties

### blockCount

> **blockCount**: [`PoolBlockCounts`](PoolBlockCounts.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:909](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L909)

Block counts for different time periods

***

### blockShare

> **blockShare**: [`PoolBlockShares`](PoolBlockShares.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:910](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L910)

Pool's share of total blocks for different time periods

***

### estimatedHashrate

> **estimatedHashrate**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:911](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L911)

Estimated hashrate based on blocks mined (H/s)

***

### pool

> **pool**: [`PoolDetailInfo`](PoolDetailInfo.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:908](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L908)

Pool information

***

### reportedHashrate?

> `optional` **reportedHashrate?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:912](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L912)

Self-reported hashrate (if available, H/s)

***

### totalReward?

> `optional` **totalReward?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:913](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L913)

Total reward earned by this pool (sats, all time; None for minor pools)
