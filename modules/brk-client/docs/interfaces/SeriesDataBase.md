[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesDataBase

# Interface: SeriesDataBase\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1568](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1568)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1575](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1575)

The series data

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1573](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1573)

End index (exclusive)

***

### entries

> **entries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1579](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1579)

Get [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1570](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1570)

The index type used for this query

***

### indexes

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1577](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1577)

Get index numbers

#### Returns

`number`[]

***

### isDateBased

> **isDateBased**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1576](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1576)

Whether this series uses a date-based index

***

### keys

> **keys**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1578](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1578)

Get keys as index numbers (alias for indexes)

#### Returns

`number`[]

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1574](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1574)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1572](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1572)

Start index (inclusive)

***

### toMap

> **toMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1580](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1580)

Convert to Map<index, value>

#### Returns

`Map`\<`number`, `T`\>

***

### type

> **type**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1571](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1571)

Value type (e.g. "f32", "u64", "Sats")

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1569](https://github.com/bitcoinresearchkit/brk/blob/6e8be1af2225890fe02f0a4598e3a4fa8251f535/modules/brk-client/index.js#L1569)

Version of the series data
