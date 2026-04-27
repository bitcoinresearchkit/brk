[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesDataBase

# Interface: SeriesDataBase\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1477](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1477)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1485](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1485)

The series data

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1483](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1483)

End index (exclusive)

***

### entries

> **entries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1489](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1489)

Get [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1479](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1479)

The index type used for this query

***

### indexes

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1487](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1487)

Get index numbers

#### Returns

`number`[]

***

### isDateBased

> **isDateBased**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1486](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1486)

Whether this series uses a date-based index

***

### keys

> **keys**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1488](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1488)

Get keys as index numbers (alias for indexes)

#### Returns

`number`[]

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1484](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1484)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1482](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1482)

Start index (inclusive)

***

### toMap

> **toMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1490](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1490)

Convert to Map<index, value>

#### Returns

`Map`\<`number`, `T`\>

***

### total

> **total**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1481](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1481)

Total number of data points

***

### type

> **type**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1480](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1480)

Value type (e.g. "f32", "u64", "Sats")

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1478](https://github.com/bitcoinresearchkit/brk/blob/07bc2d42b87ba62f766e714bbde023393049d4b7/modules/brk-client/index.js#L1478)

Version of the series data
