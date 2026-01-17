[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockStatus

# Interface: BlockStatus

Defined in: [Developer/brk/modules/brk-client/index.js:149](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L149)

## Properties

### height?

> `optional` **height**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:151](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L151)

Block height (only if in best chain)

***

### inBestChain

> **inBestChain**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:150](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L150)

Whether this block is in the best chain

***

### nextBest?

> `optional` **nextBest**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:152](https://github.com/bitcoinresearchkit/brk/blob/7bbf03766eae27aedef16f49caa69c9540eb91ad/modules/brk-client/index.js#L152)

Hash of the next block in the best chain (only if in best chain and not tip)
