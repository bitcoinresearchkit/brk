[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SkippedBuilder

# Interface: SkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1396](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1396)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1398](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1398)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1399](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1399)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take

> **take**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1397](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1397)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1400](https://github.com/bitcoinresearchkit/brk/blob/5bf06530ce84cd1829c2f934e5baded9c9125d45/modules/brk-client/index.js#L1400)

Thenable
