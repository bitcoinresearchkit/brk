[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / BlockStatus

# Interface: BlockStatus

Defined in: [Developer/mono/modules/bitview-client/index.js:277](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L277)

## Properties

### height?

> `optional` **height?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:279](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L279)

Block height (only if in best chain)

***

### inBestChain

> **inBestChain**: `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:278](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L278)

Whether this block is in the best chain

***

### nextBest?

> `optional` **nextBest?**: `string` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:280](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L280)

Hash of the next block in the best chain (null if tip)
