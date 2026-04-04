[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SyncStatus

# Interface: SyncStatus

Defined in: [Developer/brk/modules/brk-client/index.js:972](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L972)

## Properties

### blocksBehind

> **blocksBehind**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:976](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L976)

Number of blocks behind the tip

***

### computedHeight

> **computedHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:974](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L974)

Height of the last computed block (series)

***

### indexedHeight

> **indexedHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:973](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L973)

Height of the last indexed block

***

### lastIndexedAt

> **lastIndexedAt**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:977](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L977)

Human-readable timestamp of the last indexed block (ISO 8601)

***

### lastIndexedAtUnix

> **lastIndexedAtUnix**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:978](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L978)

Unix timestamp of the last indexed block

***

### tipHeight

> **tipHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:975](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L975)

Height of the chain tip (from Bitcoin node)
