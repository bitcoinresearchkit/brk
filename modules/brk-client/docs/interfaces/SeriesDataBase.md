[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesDataBase

# Interface: SeriesDataBase\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1390](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1390)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1398](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1398)

The series data

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1396](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1396)

End index (exclusive)

***

### entries

> **entries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1402](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1402)

Get [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1392](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1392)

The index type used for this query

***

### indexes

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1400](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1400)

Get index numbers

#### Returns

`number`[]

***

### isDateBased

> **isDateBased**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1399](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1399)

Whether this series uses a date-based index

***

### keys

> **keys**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1401](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1401)

Get keys as index numbers (alias for indexes)

#### Returns

`number`[]

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1397](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1397)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1395](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1395)

Start index (inclusive)

***

### toMap

> **toMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1403](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1403)

Convert to Map<index, value>

#### Returns

`Map`\<`number`, `T`\>

***

### total

> **total**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1394](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1394)

Total number of data points

***

### type

> **type**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1393](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1393)

Value type (e.g. "f32", "u64", "Sats")

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1391](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L1391)

Version of the series data
