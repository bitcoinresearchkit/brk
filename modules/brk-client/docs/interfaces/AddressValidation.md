[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddressValidation

# Interface: AddressValidation

Defined in: [Developer/brk/modules/brk-client/index.js:56](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L56)

## Properties

### address?

> `optional` **address**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:58](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L58)

The validated address

***

### isscript?

> `optional` **isscript**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:60](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L60)

Whether this is a script address (P2SH)

***

### isvalid

> **isvalid**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:57](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L57)

Whether the address is valid

***

### iswitness?

> `optional` **iswitness**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:61](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L61)

Whether this is a witness address

***

### scriptPubKey?

> `optional` **scriptPubKey**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:59](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L59)

The scriptPubKey in hex

***

### witnessProgram?

> `optional` **witnessProgram**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:63](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L63)

Witness program in hex

***

### witnessVersion?

> `optional` **witnessVersion**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:62](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L62)

Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
