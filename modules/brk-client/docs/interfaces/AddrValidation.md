[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrValidation

# Interface: AddrValidation

Defined in: [Developer/brk/modules/brk-client/index.js:62](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L62)

## Properties

### address?

> `optional` **address?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:64](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L64)

The validated address

***

### error?

> `optional` **error?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:71](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L71)

Error message for invalid addresses

***

### errorLocations?

> `optional` **errorLocations?**: `number`[] \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:70](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L70)

Error locations (empty array for most errors)

***

### isscript?

> `optional` **isscript?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:66](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L66)

Whether this is a script address (P2SH)

***

### isvalid

> **isvalid**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:63](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L63)

Whether the address is valid

***

### iswitness?

> `optional` **iswitness?**: `boolean` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:67](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L67)

Whether this is a witness address

***

### scriptPubKey?

> `optional` **scriptPubKey?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:65](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L65)

The scriptPubKey in hex

***

### witnessProgram?

> `optional` **witnessProgram?**: `string` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:69](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L69)

Witness program in hex

***

### witnessVersion?

> `optional` **witnessVersion?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:68](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L68)

Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
