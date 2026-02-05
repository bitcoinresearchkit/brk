[**brk-client**](../README.md)

***

[brk-client](../globals.md) / RangeBuilder

# Interface: RangeBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1062](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L1062)

## Type Parameters

### T

`T`

## Properties

### fetch()

> **fetch**: (`onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1063](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L1063)

Fetch the range

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

***

### fetchCsv()

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1064](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L1064)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1065](https://github.com/bitcoinresearchkit/brk/blob/afe4123a1799221536f346575595e85e3b7040d4/modules/brk-client/index.js#L1065)

Thenable
