[**brk-client**](../README.md)

***

[brk-client](../globals.md) / ReplacementNode

# Interface: ReplacementNode

Defined in: [Developer/brk/modules/brk-client/index.js:1030](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1030)

## Properties

### fullRbf

> **fullRbf**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1034](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1034)

Any predecessor in this subtree was non-signaling.

***

### interval?

> `optional` **interval?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1035](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1035)

Seconds between this node's `time` and the successor that
replaced it. Omitted on the root of an RBF response.

***

### mined?

> `optional` **mined?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1037](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1037)

`Some(true)` iff this node's tx is currently confirmed. Absent
on serialization otherwise.

***

### replaces

> **replaces**: `ReplacementNode`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1039](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1039)

***

### time

> **time**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1032](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1032)

First-seen timestamp, duplicated here to match mempool.space's
on-the-wire shape.

***

### tx

> **tx**: [`RbfTx`](RbfTx.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1031](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1031)
