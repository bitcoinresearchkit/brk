[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SkippedBuilder

# Interface: SkippedBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1015](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1015)

## Type Parameters

### T

`T`

## Properties

### fetch()

> **fetch**: (`onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1017](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1017)

Fetch from skipped position to end

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

***

### fetchCsv()

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1018](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1018)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### take()

> **take**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1016](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1016)

Take n items after skipped position

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1019](https://github.com/bitcoinresearchkit/brk/blob/0433e3b25662fded3395ef89ebe1c68d82b918b1/modules/brk-client/index.js#L1019)

Thenable
