[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockTemplate

# Interface: BlockTemplate

Defined in: [Developer/brk/modules/brk-client/index.js:289](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L289)

## Properties

### hash

> **hash**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:290](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L290)

Pass back as `<hash>` on
`/api/v1/mempool/block-template/diff/{hash}` to fetch deltas.

***

### stats

> **stats**: [`MempoolBlock`](MempoolBlock.md)

Defined in: [Developer/brk/modules/brk-client/index.js:292](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L292)

Aggregate stats for this block (size, vsize, fee range, ...).

***

### transactions

> **transactions**: [`Transaction`](Transaction.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:293](https://github.com/bitcoinresearchkit/brk/blob/d161462137a58a76f972f5e15b0029af02a165ca/modules/brk-client/index.js#L293)

Full transaction bodies in `getblocktemplate` order.
