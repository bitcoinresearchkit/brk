[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSingleItemBuilder

# Interface: DateSingleItemBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1211](https://github.com/bitcoinresearchkit/brk/blob/19d4a193ffb7d8bbcaf9d0ccaccf8e90edff9e28/modules/brk-client/index.js#L1211)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1212](https://github.com/bitcoinresearchkit/brk/blob/19d4a193ffb7d8bbcaf9d0ccaccf8e90edff9e28/modules/brk-client/index.js#L1212)

Fetch the item

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1213](https://github.com/bitcoinresearchkit/brk/blob/19d4a193ffb7d8bbcaf9d0ccaccf8e90edff9e28/modules/brk-client/index.js#L1213)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1214](https://github.com/bitcoinresearchkit/brk/blob/19d4a193ffb7d8bbcaf9d0ccaccf8e90edff9e28/modules/brk-client/index.js#L1214)

Thenable
