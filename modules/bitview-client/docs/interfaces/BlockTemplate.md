[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / BlockTemplate

# Interface: BlockTemplate

Defined in: [Developer/mono/modules/bitview-client/index.js:287](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L287)

## Properties

### hash

> **hash**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:288](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L288)

Pass to `GET /api/v1/mempool/block-template/diff/{hash}` to fetch deltas.

***

### stats

> **stats**: [`MempoolBlock`](MempoolBlock.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:289](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L289)

Aggregate stats for this block (size, vsize, fee range, ...).

***

### transactions

> **transactions**: [`Transaction`](Transaction.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:290](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L290)

Full transaction bodies in `getblocktemplate` order.
