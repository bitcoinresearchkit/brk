[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RbfTx

# Interface: RbfTx

Defined in: [Developer/brk/modules/brk-client/index.js:923](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L923)

## Properties

### fee

> **fee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:925](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L925)

***

### fullRbf?

> `optional` **fullRbf?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:931](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L931)

Only populated on the root `tx` of an RBF response. `true` iff
this tx displaced at least one non-signaling predecessor.

***

### mined?

> `optional` **mined?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:933](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L933)

`Some(true)` iff the tx is currently confirmed in the indexed
chain. Absent on serialization when the tx is still pending or
has been evicted without confirming.

***

### rate

> **rate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:928](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L928)

***

### rbf

> **rbf**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:930](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L930)

BIP-125 signaling: at least one input has sequence < 0xffffffff-1.

***

### time

> **time**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:929](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L929)

***

### txid

> **txid**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:924](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L924)

***

### value

> **value**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:927](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L927)

Sum of output amounts.

***

### vsize

> **vsize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:926](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L926)
