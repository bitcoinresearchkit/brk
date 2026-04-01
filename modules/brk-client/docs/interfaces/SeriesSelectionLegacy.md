[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesSelectionLegacy

# Interface: SeriesSelectionLegacy

Defined in: [Developer/brk/modules/brk-client/index.js:910](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L910)

## Properties

### end?

> `optional` **end?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:914](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L914)

Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`

***

### format?

> `optional` **format?**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:916](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L916)

Format of the output

***

### ids

> **ids**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:912](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L912)

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:911](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L911)

***

### limit?

> `optional` **limit?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:915](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L915)

Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`

***

### start?

> `optional` **start?**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:913](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L913)

Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
