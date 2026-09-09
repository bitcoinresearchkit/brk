[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / addressPayloadHashPrefix

# Function: addressPayloadHashPrefix()

> **addressPayloadHashPrefix**(`payload`, `nibbles`): `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:2355](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L2355)

Compute the RapidHash v3 hash-prefix used by `/api/address/hash-prefix/{addr_type}/{prefix}`.

## Parameters

### payload

`number`[] \| `ArrayBuffer` \| `Uint8Array`\<`ArrayBufferLike`\> \| `ArrayBufferView`\<`ArrayBufferLike`\>

Raw address payload bytes

### nibbles

`number`

Prefix length from 1 to 16 hex nibbles

## Returns

`string`
