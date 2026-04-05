[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkError

# Class: BrkError

Defined in: [Developer/brk/modules/brk-client/index.js:1205](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1205)

Custom error class for BRK client errors

## Extends

- `Error`

## Constructors

### Constructor

> **new BrkError**(`message`, `status?`): `BrkError`

Defined in: [Developer/brk/modules/brk-client/index.js:1210](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L1210)

#### Parameters

##### message

`string`

##### status?

`number`

#### Returns

`BrkError`

#### Overrides

`Error.constructor`

## Methods

### isError()

> `static` **isError**(`error`): `error is Error`

Defined in: .npm/\_npx/940582f83630445a/node\_modules/typescript/lib/lib.esnext.error.d.ts:23

Indicates whether the argument provided is a built-in Error instance or not.

#### Parameters

##### error

`unknown`

#### Returns

`error is Error`

#### Inherited from

`Error.isError`
