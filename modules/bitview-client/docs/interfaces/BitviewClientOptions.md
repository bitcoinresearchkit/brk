[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / BitviewClientOptions

# Interface: BitviewClientOptions

Defined in: [Developer/mono/modules/bitview-client/index.js:1501](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1501)

## Properties

### baseUrl

> **baseUrl**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1502](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1502)

Base URL for the API

***

### browserCache?

> `optional` **browserCache?**: `string` \| `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1504](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1504)

Enable browser Cache API with default name (true), custom name (string), or disable (false). No effect in Node.js. Default: true

***

### memCache?

> `optional` **memCache?**: `number` \| `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1505](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1505)

In-memory parsed-response cache size (LRU). true/undefined → 1000, false/0 → disabled. Lets 304 responses skip the JSON parse entirely. Default: 1000

***

### timeout?

> `optional` **timeout?**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1503](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1503)

Request timeout in milliseconds
