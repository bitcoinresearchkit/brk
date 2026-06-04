[**brk-client**](../README.md)

***

[brk-client](../globals.md) / UrpdBucket

# Interface: UrpdBucket

Defined in: [Developer/brk/modules/brk-client/index.js:1367](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1367)

## Properties

### priceFloor

> **priceFloor**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1368](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1368)

Lower bound of the bucket, in USD. Equals the exact realized price for `Raw`.

***

### realizedCap

> **realizedCap**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1370](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1370)

Realized cap contribution in USD: sum of `realized_price * supply` over the coins in this bucket.

***

### supply

> **supply**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1369](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1369)

Supply held with a last-move price inside this bucket, in BTC.

***

### unrealizedPnl

> **unrealizedPnl**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1371](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1371)

Unrealized P&L in USD against the close on the snapshot date: `close * supply - realized_cap`. Can be negative.
