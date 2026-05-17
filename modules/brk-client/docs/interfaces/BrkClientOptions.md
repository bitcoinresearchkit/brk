[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClientOptions

# Interface: BrkClientOptions

Defined in: [Developer/brk/modules/brk-client/index.js:1442](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1442)

## Properties

### baseUrl

> **baseUrl**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1443](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1443)

Base URL for the API

***

### browserCache?

> `optional` **browserCache?**: `string` \| `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1445](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1445)

Enable browser Cache API with default name (true), custom name (string), or disable (false). No effect in Node.js. Default: true

***

### memCache?

> `optional` **memCache?**: `number` \| `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1446](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1446)

In-memory parsed-response cache size (LRU). true/undefined → 1000, false/0 → disabled. Lets 304 responses skip the JSON parse entirely. Default: 1000

***

### timeout?

> `optional` **timeout?**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1444](https://github.com/bitcoinresearchkit/brk/blob/1a706da13cc492eee123fc28fd358f02b56918b6/modules/brk-client/index.js#L1444)

Request timeout in milliseconds
