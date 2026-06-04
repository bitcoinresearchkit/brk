[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateRangeBuilder

# Interface: DateRangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1732](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1732)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onValue?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1733](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1733)

Fetch the range

#### Parameters

##### onValue?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1734](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1734)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1735](https://github.com/bitcoinresearchkit/brk/blob/37e2b6eae2ee7db79b2d392e73eb0697e4a91b28/modules/brk-client/index.js#L1735)

Thenable
