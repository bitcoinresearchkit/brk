[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockStatus

# Interface: BlockStatus

Defined in: [Developer/brk/modules/brk-client/index.js:175](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L175)

## Properties

### height?

> `optional` **height?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:177](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L177)

Block height (only if in best chain)

***

### inBestChain

> **inBestChain**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:176](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L176)

Whether this block is in the best chain

***

### nextBest?

> `optional` **nextBest?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:178](https://github.com/bitcoinresearchkit/brk/blob/041652d85d1f6ed0371402865bf343ad227ee250/modules/brk-client/index.js#L178)

Hash of the next block in the best chain (only if in best chain and not tip)
