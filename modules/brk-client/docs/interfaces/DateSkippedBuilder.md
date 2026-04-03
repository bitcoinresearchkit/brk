[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSkippedBuilder

# Interface: DateSkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1404](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1404)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1406](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1406)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1407](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1407)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take

> **take**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1405](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1405)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1408](https://github.com/bitcoinresearchkit/brk/blob/883b38c77cb9f979692884fd56d9966e0f8ea76b/modules/brk-client/index.js#L1408)

Thenable
