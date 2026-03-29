[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DataRangeFormat

# Interface: DataRangeFormat

Defined in: [Developer/brk/modules/brk-client/index.js:267](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L267)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:269](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L269)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:271](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L271)

Format of the output

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:270](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L270)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:268](https://github.com/bitcoinresearchkit/brk/blob/d4dc1b9e4900e3787f2a133b8cac5d304acff9bf/modules/brk-client/index.js#L268)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
