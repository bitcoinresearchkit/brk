[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesSelection

# Interface: SeriesSelection

Defined in: [Developer/brk/modules/brk-client/index.js:1114](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1114)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1118](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1118)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1120](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1120)

Format of the output

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1116](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1116)

Index to query

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1119](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1119)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### series

> **series**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1115](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1115)

Requested series

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:1117](https://github.com/bitcoinresearchkit/brk/blob/e23554811be77a28474c946e0b8ab586d4075ec9/modules/brk-client/index.js#L1117)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
