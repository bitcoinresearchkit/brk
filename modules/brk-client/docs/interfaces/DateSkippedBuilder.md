[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSkippedBuilder

# Interface: DateSkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1224](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1224)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1226](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1226)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1227](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1227)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take

> **take**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1225](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1225)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1228](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L1228)

Thenable
