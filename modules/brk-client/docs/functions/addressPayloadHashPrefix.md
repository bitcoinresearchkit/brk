[**brk-client**](../README.md)

***

[brk-client](../globals.md) / addressPayloadHashPrefix

# Function: addressPayloadHashPrefix()

> **addressPayloadHashPrefix**(`payload`, `nibbles`): `string`

Compute the RapidHash v3 hash-prefix used by `/api/address/hash-prefix/{addr_type}/{prefix}`.

## Parameters

### payload

`Uint8Array` | `ArrayBuffer` | `ArrayBufferView` | `number`[]

Raw address payload bytes. Must be 1 to 65 bytes.

### nibbles

`number`

Prefix length from 1 to 16 hex nibbles.

## Returns

`string`

