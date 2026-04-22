[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesSelectionLegacy

# Interface: SeriesSelectionLegacy

Defined in: [Developer/brk/modules/brk-client/index.js:968](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L968)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:972](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L972)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:974](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L974)

Format of the output

***

### ids

> **ids**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:970](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L970)

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:969](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L969)

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:973](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L973)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:971](https://github.com/bitcoinresearchkit/brk/blob/bdc3ba1df62c3c70339afe14f39c8c9a5a094c37/modules/brk-client/index.js#L971)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
