[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / RbfTx

# Interface: RbfTx

Defined in: [Developer/mono/modules/bitview-client/index.js:1028](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1028)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1030](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1030)

***

### fullRbf?

> `optional` **fullRbf?**: `boolean` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1036](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1036)

Only populated on the root `tx` of an RBF response. `true` iff
this tx displaced at least one non-signaling predecessor.

***

### rate

> **rate**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1033](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1033)

***

### rbf

> **rbf**: `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1035](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1035)

BIP-125 signaling: at least one input has sequence < 0xffffffff-1.

***

### time

> **time**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1034](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1034)

***

### txid

> **txid**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1029](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1029)

***

### value

> **value**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1032](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1032)

Sum of output amounts.

***

### vsize

> **vsize**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1031](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1031)
