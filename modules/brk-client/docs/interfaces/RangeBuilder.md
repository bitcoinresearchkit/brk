[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RangeBuilder

# Interface: RangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1726](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1726)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onValue?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1727](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1727)

Fetch the range

#### Parameters

##### onValue?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1728](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1728)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1729](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1729)

Thenable
