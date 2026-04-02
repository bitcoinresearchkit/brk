[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockStatus

# Interface: BlockStatus

Defined in: [Developer/brk/modules/brk-client/index.js:244](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L244)

## Properties

### height?

> `optional` **height?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:246](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L246)

Block height (only if in best chain)

***

### inBestChain

> **inBestChain**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:245](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L245)

Whether this block is in the best chain

***

### nextBest?

> `optional` **nextBest?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:247](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L247)

Hash of the next block in the best chain (null if tip)
