[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RbfTx

# Interface: RbfTx

Defined in: [Developer/brk/modules/brk-client/index.js:1005](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1005)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1007](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1007)

***

### fullRbf?

> `optional` **fullRbf?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1013](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1013)

Only populated on the root `tx` of an RBF response. `true` iff
this tx displaced at least one non-signaling predecessor.

***

### rate

> **rate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1010](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1010)

***

### rbf

> **rbf**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1012](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1012)

BIP-125 signaling: at least one input has sequence < 0xffffffff-1.

***

### time

> **time**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1011](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1011)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1006](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1006)

***

### value

> **value**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1009](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1009)

Sum of output amounts.

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1008](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L1008)
