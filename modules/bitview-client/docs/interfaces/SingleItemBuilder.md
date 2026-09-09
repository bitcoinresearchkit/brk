[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / SingleItemBuilder

# Interface: SingleItemBuilder\<T\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1769](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1769)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`arg?`, `options?`) => `Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1770](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1770)

Fetch the item

#### Parameters

##### arg?

[`SeriesFetchArg`](../type-aliases/SeriesFetchArg.md)\<`T`\>

##### options?

[`ClientFetchOptions`](ClientFetchOptions.md)\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

#### Returns

`Promise`\<[`SeriesData`](../type-aliases/SeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: (`options?`) => `Promise`\<`string`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1771](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1771)

Fetch as CSV

#### Parameters

##### options?

[`ClientFetchOptions`](ClientFetchOptions.md)\<`string`\>

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1772](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1772)

Thenable
