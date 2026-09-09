[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / BlockTemplateDiff

# Interface: BlockTemplateDiff

Defined in: [Developer/mono/modules/bitview-client/index.js:307](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L307)

## Properties

### hash

> **hash**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:308](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L308)

Current next-block hash. Use as `since` on the next diff call.

***

### order

> **order**: [`BlockTemplateDiffEntry`](../type-aliases/BlockTemplateDiffEntry.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:310](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L310)

New template in order. Each entry is either an index into the
prior template's transactions or a full transaction body.

***

### removed

> **removed**: `string`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:312](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L312)

Txids that left the projected next block since `since`
(confirmed, evicted, replaced, or pushed past block 0).

***

### since

> **since**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:309](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L309)

Echoed prior hash the diff was computed against.
