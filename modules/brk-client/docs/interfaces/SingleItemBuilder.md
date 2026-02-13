[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SingleItemBuilder

# Interface: SingleItemBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1045](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1045)

## Type Parameters

### T

`T`

## Properties

### fetch()

> **fetch**: (`onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1046](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1046)

Fetch the item

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

***

### fetchCsv()

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1047](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1047)

Fetch as CSV

#### Returns

`Promise`\<`string`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1048](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L1048)

Thenable
