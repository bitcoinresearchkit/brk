[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrValidation

# Interface: AddrValidation

Defined in: [Developer/brk/modules/brk-client/index.js:74](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L74)

## Properties

### address?

> `optional` **address?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:76](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L76)

The validated address

***

### error?

> `optional` **error?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:83](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L83)

Error message for invalid addresses

***

### errorLocations?

> `optional` **errorLocations?**: `number`[] \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:82](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L82)

Error locations (empty array for most errors)

***

### isscript?

> `optional` **isscript?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:78](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L78)

Whether this is a script address (P2SH)

***

### isvalid

> **isvalid**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:75](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L75)

Whether the address is valid

***

### iswitness?

> `optional` **iswitness?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:79](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L79)

Whether this is a witness address

***

### scriptPubKey?

> `optional` **scriptPubKey?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:77](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L77)

The scriptPubKey in hex

***

### witnessProgram?

> `optional` **witnessProgram?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:81](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L81)

Witness program in hex

***

### witnessVersion?

> `optional` **witnessVersion?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:80](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L80)

Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
