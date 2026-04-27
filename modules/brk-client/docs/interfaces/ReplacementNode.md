[**brk-client**](../README.md)

***

[brk-client](../globals.md) / ReplacementNode

# Interface: ReplacementNode

Defined in: [Developer/brk/modules/brk-client/index.js:887](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L887)

## Properties

### fullRbf

> **fullRbf**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:891](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L891)

Any predecessor in this subtree was non-signaling.

***

### interval?

> `optional` **interval?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:892](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L892)

Seconds between this node's `time` and the successor that
replaced it. Omitted on the root of an RBF response.

***

### replaces

> **replaces**: `ReplacementNode`[]

Defined in: [Developer/brk/modules/brk-client/index.js:894](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L894)

***

### time

> **time**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:889](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L889)

First-seen timestamp, duplicated here to match mempool.space's
on-the-wire shape.

***

### tx

> **tx**: [`RbfTx`](RbfTx.md)

Defined in: [Developer/brk/modules/brk-client/index.js:888](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L888)
