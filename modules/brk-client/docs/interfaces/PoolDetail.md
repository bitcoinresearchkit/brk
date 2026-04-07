[**brk-client**](../README.md)

***

[brk-client](../globals.md) / PoolDetail

# Interface: PoolDetail

Defined in: [Developer/brk/modules/brk-client/index.js:765](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L765)

## Properties

### blockCount

> **blockCount**: [`PoolBlockCounts`](PoolBlockCounts.md)

Defined in: [Developer/brk/modules/brk-client/index.js:767](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L767)

Block counts for different time periods

***

### blockShare

> **blockShare**: [`PoolBlockShares`](PoolBlockShares.md)

Defined in: [Developer/brk/modules/brk-client/index.js:768](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L768)

Pool's share of total blocks for different time periods

***

### estimatedHashrate

> **estimatedHashrate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:769](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L769)

Estimated hashrate based on blocks mined (H/s)

***

### pool

> **pool**: [`PoolDetailInfo`](PoolDetailInfo.md)

Defined in: [Developer/brk/modules/brk-client/index.js:766](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L766)

Pool information

***

### reportedHashrate?

> `optional` **reportedHashrate?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:770](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L770)

Self-reported hashrate (if available, H/s)

***

### totalReward?

> `optional` **totalReward?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:771](https://github.com/bitcoinresearchkit/brk/blob/c76b149ef9f4e0092fec3c6aef562bc4dddd8122/modules/brk-client/index.js#L771)

Total reward earned by this pool (sats, all time; None for minor pools)
