[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockStatus

# Interface: BlockStatus

Defined in: [Developer/brk/modules/brk-client/index.js:149](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L149)

## Properties

### height?

> `optional` **height**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:151](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L151)

Block height (only if in best chain)

***

### inBestChain

> **inBestChain**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:150](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L150)

Whether this block is in the best chain

***

### nextBest?

> `optional` **nextBest**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:152](https://github.com/bitcoinresearchkit/brk/blob/247d3c758bf072fffa634188454ed826d0eba9f2/modules/brk-client/index.js#L152)

Hash of the next block in the best chain (only if in best chain and not tip)
