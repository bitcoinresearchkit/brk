[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSkippedBuilder

# Interface: DateSkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1406](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1406)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1408](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1408)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1409](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1409)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take

> **take**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1407](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1407)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1410](https://github.com/bitcoinresearchkit/brk/blob/8bc993ecebee68170d873d232b25f3b1f2d71378/modules/brk-client/index.js#L1410)

Thenable
