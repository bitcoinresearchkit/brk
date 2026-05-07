[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClientOptions

# Interface: BrkClientOptions

Defined in: [Developer/brk/modules/brk-client/index.js:1383](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1383)

## Properties

### baseUrl

> **baseUrl**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1384](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1384)

Base URL for the API

***

### browserCache?

> `optional` **browserCache?**: `string` \| `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1386](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1386)

Enable browser Cache API with default name (true), custom name (string), or disable (false). No effect in Node.js. Default: true

***

### memCache?

> `optional` **memCache?**: `number` \| `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1387](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1387)

In-memory parsed-response cache size (LRU). true/undefined → 1000, false/0 → disabled. Lets 304 responses skip the JSON parse entirely. Default: 1000

***

### timeout?

> `optional` **timeout?**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1385](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1385)

Request timeout in milliseconds
