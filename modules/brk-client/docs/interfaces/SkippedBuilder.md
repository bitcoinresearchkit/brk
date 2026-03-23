[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SkippedBuilder

# Interface: SkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1217](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1217)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1219](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1219)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1220](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1220)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take

> **take**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1218](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1218)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1221](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L1221)

Thenable
