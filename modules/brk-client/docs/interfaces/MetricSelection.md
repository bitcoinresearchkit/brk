[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricSelection

# Interface: MetricSelection

Defined in: [Developer/brk/modules/brk-client/index.js:395](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L395)

## Properties

### end?

> `optional` **end**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:399](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L399)

Exclusive ending index, if negative counts from end

***

### format?

> `optional` **format**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:401](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L401)

Format of the output

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:397](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L397)

Index to query

***

### limit?

> `optional` **limit**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:400](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L400)

Maximum number of values to return (ignored if `end` is set)

***

### metrics

> **metrics**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:396](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L396)

Requested metrics

***

### start?

> `optional` **start**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:398](https://github.com/bitcoinresearchkit/brk/blob/79f7e89740d35d2bbc56505cbbcf3e6a4fe4a0f3/modules/brk-client/index.js#L398)

Inclusive starting index, if negative counts from end
