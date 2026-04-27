[**brk-client**](../README.md)

***

[brk-client](../globals.md) / UrpdBucket

# Interface: UrpdBucket

Defined in: [Developer/brk/modules/brk-client/index.js:1213](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1213)

## Properties

### priceFloor

> **priceFloor**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1214](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1214)

Inclusive lower bound of the bucket, in USD.

***

### realizedCap

> **realizedCap**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1216](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1216)

Realized cap contribution in USD: `price_floor * supply`.

***

### supply

> **supply**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1215](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1215)

Supply held with a last-move price inside this bucket, in BTC.

***

### unrealizedPnl

> **unrealizedPnl**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1217](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1217)

Unrealized P&L in USD against the close on the snapshot date: `(close - price_floor) * supply`. Can be negative.
