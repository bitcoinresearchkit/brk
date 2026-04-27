[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RbfTx

# Interface: RbfTx

Defined in: [Developer/brk/modules/brk-client/index.js:862](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L862)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:864](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L864)

***

### fullRbf?

> `optional` **fullRbf?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:870](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L870)

Only populated on the root `tx` of an RBF response. `true` iff
this tx displaced at least one non-signaling predecessor.

***

### rate

> **rate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:867](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L867)

***

### rbf

> **rbf**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:869](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L869)

BIP-125 signaling: at least one input has sequence < 0xffffffff-1.

***

### time

> **time**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:868](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L868)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:863](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L863)

***

### value

> **value**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:866](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L866)

Sum of output amounts.

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:865](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L865)
