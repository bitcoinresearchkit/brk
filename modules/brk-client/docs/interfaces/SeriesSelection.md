[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesSelection

# Interface: SeriesSelection

Defined in: [Developer/brk/modules/brk-client/index.js:957](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L957)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:961](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L961)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:963](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L963)

Format of the output

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:959](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L959)

Index to query

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:962](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L962)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### series

> **series**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:958](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L958)

Requested series

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:960](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L960)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
