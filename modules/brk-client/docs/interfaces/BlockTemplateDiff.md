[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BlockTemplateDiff

# Interface: BlockTemplateDiff

Defined in: [Developer/brk/modules/brk-client/index.js:321](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L321)

## Properties

### hash

> **hash**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:322](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L322)

Current next-block hash. Use as `since` on the next diff call.

***

### order

> **order**: [`BlockTemplateDiffEntry`](../type-aliases/BlockTemplateDiffEntry.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:324](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L324)

New template in order. Each entry is either an index into the
prior template's transactions or a full transaction body.

***

### removed

> **removed**: `string`[]

Defined in: [Developer/brk/modules/brk-client/index.js:326](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L326)

Txids that left the projected next block since `since`
(confirmed, evicted, replaced, or pushed past block 0).

***

### since

> **since**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:323](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L323)

Echoed prior hash the diff was computed against.
