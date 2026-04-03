[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SyncStatus

# Interface: SyncStatus

Defined in: [Developer/brk/modules/brk-client/index.js:970](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L970)

## Properties

### blocksBehind

> **blocksBehind**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:974](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L974)

Number of blocks behind the tip

***

### computedHeight

> **computedHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:972](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L972)

Height of the last computed block (series)

***

### indexedHeight

> **indexedHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:971](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L971)

Height of the last indexed block

***

### lastIndexedAt

> **lastIndexedAt**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:975](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L975)

Human-readable timestamp of the last indexed block (ISO 8601)

***

### lastIndexedAtUnix

> **lastIndexedAtUnix**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:976](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L976)

Unix timestamp of the last indexed block

***

### tipHeight

> **tipHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:973](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L973)

Height of the chain tip (from Bitcoin node)
