[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RangeBuilder

# Interface: RangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1718](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1718)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onValue?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1719](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1719)

Fetch the range

#### Parameters

##### onValue?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1720](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1720)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1721](https://github.com/bitcoinresearchkit/brk/blob/6ff43c0f74cf0925ed63288a681f08b5cb45400b/modules/brk-client/index.js#L1721)

Thenable
