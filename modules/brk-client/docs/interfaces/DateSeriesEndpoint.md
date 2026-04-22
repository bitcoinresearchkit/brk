[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DateSeriesEndpoint

# Interface: DateSeriesEndpoint\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1438](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1438)

## Type Parameters

### T

`T`

## Properties

### fetch

> **fetch**: (`onUpdate?`) => `Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:1444](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1444)

Fetch all data

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`DateSeriesData`](../type-aliases/DateSeriesData.md)\<`T`\>\>

***

### fetchCsv

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1445](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1445)

Fetch all data as CSV

#### Returns

`Promise`\<`string`\>

***

### first

> **first**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1441](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1441)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### get

> **get**: (`index`) => [`DateSingleItemBuilder`](DateSingleItemBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1439](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1439)

Get single item at index or Date

#### Parameters

##### index

`number` \| `Date`

#### Returns

[`DateSingleItemBuilder`](DateSingleItemBuilder.md)\<`T`\>

***

### last

> **last**: (`n`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1442](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1442)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1447](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1447)

The endpoint path

***

### skip

> **skip**: (`n`) => [`DateSkippedBuilder`](DateSkippedBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1443](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1443)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`DateSkippedBuilder`](DateSkippedBuilder.md)\<`T`\>

***

### slice

> **slice**: (`start?`, `end?`) => [`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1440](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1440)

Slice by index or Date

#### Parameters

##### start?

`number` \| `Date`

##### end?

`number` \| `Date`

#### Returns

[`DateRangeBuilder`](DateRangeBuilder.md)\<`T`\>

***

### then

> **then**: [`DateThenable`](../type-aliases/DateThenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1446](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1446)

Thenable (await endpoint)
