[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrValidation

# Interface: AddrValidation

Defined in: [Developer/brk/modules/brk-client/index.js:55](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L55)

## Properties

### address?

> `optional` **address?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:57](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L57)

The validated address

***

### error?

> `optional` **error?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:64](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L64)

Error message for invalid addresses

***

### errorLocations?

> `optional` **errorLocations?**: `number`[] \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:63](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L63)

Error locations (empty array for most errors)

***

### isscript?

> `optional` **isscript?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:59](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L59)

Whether this is a script address (P2SH)

***

### isvalid

> **isvalid**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:56](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L56)

Whether the address is valid

***

### iswitness?

> `optional` **iswitness?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:60](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L60)

Whether this is a witness address

***

### scriptPubKey?

> `optional` **scriptPubKey?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:58](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L58)

The scriptPubKey in hex

***

### witnessProgram?

> `optional` **witnessProgram?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:62](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L62)

Witness program in hex

***

### witnessVersion?

> `optional` **witnessVersion?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:61](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L61)

Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
