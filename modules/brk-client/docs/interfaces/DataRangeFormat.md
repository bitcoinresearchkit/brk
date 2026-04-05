[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DataRangeFormat

# Interface: DataRangeFormat

Defined in: [Developer/brk/modules/brk-client/index.js:385](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L385)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:387](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L387)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:389](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L389)

Format of the output

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:388](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L388)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:386](https://github.com/bitcoinresearchkit/brk/blob/acd3d6f42524ece8c85e7b09cc9fdb13b5687b9f/modules/brk-client/index.js#L386)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
