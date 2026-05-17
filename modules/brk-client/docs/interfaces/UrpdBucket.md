[**brk-client**](../README.md)

***

[brk-client](../globals.md) / UrpdBucket

# Interface: UrpdBucket

Defined in: [Developer/brk/modules/brk-client/index.js:1359](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1359)

## Properties

### priceFloor

> **priceFloor**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1360](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1360)

Lower bound of the bucket, in USD. Equals the exact realized price for `Raw`.

***

### realizedCap

> **realizedCap**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1362](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1362)

Realized cap contribution in USD: sum of `realized_price * supply` over the coins in this bucket.

***

### supply

> **supply**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1361](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1361)

Supply held with a last-move price inside this bucket, in BTC.

***

### unrealizedPnl

> **unrealizedPnl**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1363](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1363)

Unrealized P&L in USD against the close on the snapshot date: `close * supply - realized_cap`. Can be negative.
