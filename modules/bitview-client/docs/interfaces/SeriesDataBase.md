[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / SeriesDataBase

# Interface: SeriesDataBase\<T\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1692](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1692)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1699](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1699)

The series data

***

### end

> **end**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1697](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1697)

End index (exclusive)

***

### entries

> **entries**: () => \[`number`, `T`\][]

Defined in: [Developer/mono/modules/bitview-client/index.js:1703](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1703)

Get [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/mono/modules/bitview-client/index.js:1694](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1694)

The index type used for this query

***

### indexes

> **indexes**: () => `number`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1701](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1701)

Get index numbers

#### Returns

`number`[]

***

### isDateBased

> **isDateBased**: `boolean`

Defined in: [Developer/mono/modules/bitview-client/index.js:1700](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1700)

Whether this series uses a date-based index

***

### keys

> **keys**: () => `number`[]

Defined in: [Developer/mono/modules/bitview-client/index.js:1702](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1702)

Get keys as index numbers (alias for indexes)

#### Returns

`number`[]

***

### stamp

> **stamp**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1698](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1698)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1696](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1696)

Start index (inclusive)

***

### toMap

> **toMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/mono/modules/bitview-client/index.js:1704](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1704)

Convert to Map<index, value>

#### Returns

`Map`\<`number`, `T`\>

***

### type

> **type**: `string`

Defined in: [Developer/mono/modules/bitview-client/index.js:1695](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1695)

Value type (e.g. "f32", "u64", "Sats")

***

### version

> **version**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:1693](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L1693)

Version of the series data
