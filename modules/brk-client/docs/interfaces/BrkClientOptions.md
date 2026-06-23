[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkClientOptions

# Interface: BrkClientOptions

Defined in: [Developer/brk/modules/brk-client/index.js:1462](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1462)

## Properties

### baseUrl

> **baseUrl**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1463](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1463)

Base URL for the API

***

### browserCache?

> `optional` **browserCache?**: `string` \| `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1465](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1465)

Enable browser Cache API with default name (true), custom name (string), or disable (false). No effect in Node.js. Default: true

***

### memCache?

> `optional` **memCache?**: `number` \| `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1466](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1466)

In-memory parsed-response cache size (LRU). true/undefined → 1000, false/0 → disabled. Lets 304 responses skip the JSON parse entirely. Default: 1000

***

### timeout?

> `optional` **timeout?**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1464](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1464)

Request timeout in milliseconds
