[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / UrpdBucket

# Interface: UrpdBucket

Defined in: [Developer/mono/modules/bitview-client/index.js:1401](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1401)

## Properties

### priceFloor

> **priceFloor**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1402](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1402)

Lower bound of the bucket, in USD. Equals the exact realized price for `Raw`.

***

### realizedCap

> **realizedCap**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1404](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1404)

Realized cap contribution in USD: sum of `realized_price * supply` over the coins in this bucket.

***

### supply

> **supply**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1403](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1403)

Supply held with a last-move price inside this bucket, in BTC.

***

### unrealizedPnl

> **unrealizedPnl**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1405](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1405)

Unrealized P&L in USD against the close on the snapshot date: `close * supply - realized_cap`. Can be negative.
