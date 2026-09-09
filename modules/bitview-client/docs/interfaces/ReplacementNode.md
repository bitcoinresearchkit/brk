[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / ReplacementNode

# Interface: ReplacementNode

Defined in: [Developer/mono/modules/bitview-client/index.js:1053](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1053)

## Properties

### fullRbf

> **fullRbf**: `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1057](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1057)

Any predecessor in this subtree was non-signaling.

***

### interval?

> `optional` **interval?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1058](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1058)

Seconds between this node's `time` and the successor that
replaced it. Omitted on the root of an RBF response.

***

### mined?

> `optional` **mined?**: `boolean` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1060](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1060)

`Some(true)` iff this node's tx is currently confirmed. Absent
on serialization otherwise.

***

### replaces

> **replaces**: `ReplacementNode`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1062](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1062)

***

### time

> **time**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1055](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1055)

First-seen timestamp, duplicated here to match mempool.space's
on-the-wire shape.

***

### tx

> **tx**: [`RbfTx`](RbfTx.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1054](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1054)
