[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockStatus

# Interface: BlockStatus

Defined in: [Developer/brk/modules/brk-client/index.js:279](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L279)

## Properties

### height?

> `optional` **height?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:281](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L281)

Block height (only if in best chain)

***

### inBestChain

> **inBestChain**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:280](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L280)

Whether this block is in the best chain

***

### nextBest?

> `optional` **nextBest?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:282](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L282)

Hash of the next block in the best chain (null if tip)
