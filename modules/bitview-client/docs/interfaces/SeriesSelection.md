[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / SeriesSelection

# Interface: SeriesSelection

Defined in: [Developer/mono/modules/bitview-client/index.js:1154](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1154)

## Properties

### end?

> `optional` **end?**: [`RangeIndex`](../type-aliases/RangeIndex.md) \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1158](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1158)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1160](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1160)

Format of the output

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1156](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1156)

Index to query

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1159](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1159)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### series

> **series**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1155](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1155)

Requested series

***

### start?

> `optional` **start?**: [`RangeIndex`](../type-aliases/RangeIndex.md) \| `null`

Defined in: [Developer/mono/modules/bitview-client/index.js:1157](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1157)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
