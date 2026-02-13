[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricSelection

# Interface: MetricSelection

Defined in: [Developer/brk/modules/brk-client/index.js:452](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L452)

## Properties

### end?

> `optional` **end**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:456](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L456)

Exclusive ending index, if negative counts from end

***

### format?

> `optional` **format**: [`Format`](../type-aliases/Format.md)

Defined in: [Developer/brk/modules/brk-client/index.js:458](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L458)

Format of the output

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:454](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L454)

Index to query

***

### limit?

> `optional` **limit**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:457](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L457)

Maximum number of values to return (ignored if `end` is set)

***

### metrics

> **metrics**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:453](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L453)

Requested metrics

***

### start?

> `optional` **start**: `number` \| `null`

Defined in: [Developer/brk/modules/brk-client/index.js:455](https://github.com/bitcoinresearchkit/brk/blob/4a06caec672337d1974d77b021e6d10711049dbe/modules/brk-client/index.js#L455)

Inclusive starting index, if negative counts from end
