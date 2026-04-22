[**brk-client**](../README.md)

***

[brk-client](../globals.md) / UrpdBucket

# Interface: UrpdBucket

Defined in: [Developer/brk/modules/brk-client/index.js:1175](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1175)

## Properties

### priceFloor

> **priceFloor**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1176](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1176)

Inclusive lower bound of the bucket, in USD.

***

### realizedCap

> **realizedCap**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1178](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1178)

Realized cap contribution in USD: `price_floor * supply`.

***

### supply

> **supply**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1177](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1177)

Supply held with a last-move price inside this bucket, in BTC.

***

### unrealizedPnl

> **unrealizedPnl**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1179](https://github.com/bitcoinresearchkit/brk/blob/3faa989691a1f662d8f7f98dbce612dc66a737c7/modules/brk-client/index.js#L1179)

Unrealized P&L in USD against the close on the snapshot date: `(close - price_floor) * supply`. Can be negative.
