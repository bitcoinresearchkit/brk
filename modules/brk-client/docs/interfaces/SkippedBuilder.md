[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SkippedBuilder

# Interface: SkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1381](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1381)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1383](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1383)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1384](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1384)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take

> **take**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1382](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1382)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1385](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L1385)

Thenable
