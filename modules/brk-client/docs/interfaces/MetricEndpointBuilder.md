[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricEndpointBuilder

# Interface: MetricEndpointBuilder\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:858](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L858)

## Type Parameters

### T

`T`

## Properties

### fetch()

> **fetch**: (`onUpdate?`) => `Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

Defined in: [Developer/brk/modules/brk-client/index.js:864](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L864)

Fetch all data

#### Parameters

##### onUpdate?

(`value`) => `void`

#### Returns

`Promise`\<[`MetricData`](MetricData.md)\<`T`\>\>

***

### fetchCsv()

> **fetchCsv**: () => `Promise`\<`string`\>

Defined in: [Developer/brk/modules/brk-client/index.js:865](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L865)

Fetch all data as CSV

#### Returns

`Promise`\<`string`\>

***

### first()

> **first**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:861](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L861)

Get first n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### get()

> **get**: (`index`) => [`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:859](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L859)

Get single item at index

#### Parameters

##### index

`number`

#### Returns

[`SingleItemBuilder`](SingleItemBuilder.md)\<`T`\>

***

### last()

> **last**: (`n`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:862](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L862)

Get last n items

#### Parameters

##### n

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### path

> **path**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:867](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L867)

The endpoint path

***

### skip()

> **skip**: (`n`) => [`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:863](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L863)

Skip first n items, chain with take()

#### Parameters

##### n

`number`

#### Returns

[`SkippedBuilder`](SkippedBuilder.md)\<`T`\>

***

### slice()

> **slice**: (`start?`, `end?`) => [`RangeBuilder`](RangeBuilder.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:860](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L860)

Slice like Array.slice

#### Parameters

##### start?

`number`

##### end?

`number`

#### Returns

[`RangeBuilder`](RangeBuilder.md)\<`T`\>

***

### then

> **then**: [`Thenable`](../type-aliases/Thenable.md)\<`T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:866](https://github.com/bitcoinresearchkit/brk/blob/2e1037ff3648b9362cc223c71160f3d69c7730ad/modules/brk-client/index.js#L866)

Thenable (await endpoint)
