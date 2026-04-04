[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SingleItemBuilder

# Interface: SingleItemBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1394](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L1394)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1395](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L1395)

Fetch the item

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1396](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L1396)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1397](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L1397)

Thenable
