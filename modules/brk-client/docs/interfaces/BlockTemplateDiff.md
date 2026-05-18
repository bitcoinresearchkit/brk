[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockTemplateDiff

# Interface: BlockTemplateDiff

Defined in: [Developer/brk/modules/brk-client/index.js:309](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L309)

## Properties

### hash

> **hash**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:310](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L310)

Current next-block hash. Use as `since` on the next diff call.

***

### order

> **order**: [`BlockTemplateDiffEntry`](../type-aliases/BlockTemplateDiffEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:312](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L312)

New template in order. Each entry is either an index into the
prior template's transactions or a full transaction body.

***

### removed

> **removed**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:314](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L314)

Txids that left the projected next block since `since`
(confirmed, evicted, replaced, or pushed past block 0).

***

### since

> **since**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:311](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L311)

Echoed prior hash the diff was computed against.
