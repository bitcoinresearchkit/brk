[**brk-client**](../README.md)

***

[brk-client](../globals.md) / BrkError

# Class: BrkError

Defined in: [Developer/brk/modules/brk-client/index.js:892](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L892)

Custom error class for BRK client errors

## Extends

- `Error`

## Constructors

### Constructor

> **new BrkError**(`message`, `status?`): `BrkError`

Defined in: [Developer/brk/modules/brk-client/index.js:897](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L897)

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
