[**brk-client**](../README.md)

***

[brk-client](../globals.md) / SeriesDataBase

# Interface: SeriesDataBase\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1565](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1565)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1572](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1572)

The series data

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1570](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1570)

End index (exclusive)

***

### entries

> **entries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1576](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1576)

Get [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1567](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1567)

The index type used for this query

***

### indexes

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1574](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1574)

Get index numbers

#### Returns

`number`[]

***

### isDateBased

> **isDateBased**: `boolean`

Defined in: [Developer/brk/modules/brk-client/index.js:1573](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1573)

Whether this series uses a date-based index

***

### keys

> **keys**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1575](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1575)

Get keys as index numbers (alias for indexes)

#### Returns

`number`[]

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1571](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1571)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1569](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1569)

Start index (inclusive)

***

### toMap

> **toMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1577](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1577)

Convert to Map<index, value>

#### Returns

`Map`\<`number`, `T`\>

***

### type

> **type**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1568](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1568)

Value type (e.g. "f32", "u64", "Sats")

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1566](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L1566)

Version of the series data
