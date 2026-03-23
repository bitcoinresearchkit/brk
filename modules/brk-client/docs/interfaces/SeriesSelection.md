[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesSelection

# Interface: SeriesSelection

Defined in: [Developer/brk/modules/brk-client/index.js:749](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L749)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:753](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L753)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:755](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L755)

Format of the output

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:751](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L751)

Index to query

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:754](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L754)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### series

> **series**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:750](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L750)

Requested series

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:752](https://github.com/bitcoinresearchkit/brk/blob/c4c0004c4a75c182b98b9e69b3c38d3fa6f96f0e/modules/brk-client/index.js#L752)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
