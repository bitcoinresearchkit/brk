[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricData

# Interface: MetricData\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:964](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L964)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:971](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L971)

The metric data

***

### dateEntries()

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:976](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L976)

Return data as [date, value] pairs (date-based only)

#### Returns

\[`Date`, `T`\][]

***

### dates()

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:972](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L972)

Convert index range to dates (date-based indexes only)

#### Returns

`Date`[]

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:969](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L969)

End index (exclusive)

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:966](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L966)

The index type used for this query

***

### indexEntries()

> **indexEntries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:977](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L977)

Return data as [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### indexes()

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:973](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L973)

Get index range as array

#### Returns

`number`[]

***

### iter()

> **iter**: () => `IterableIterator`\<\[`number`, `T`\]\>

Defined in: [Developer/brk/modules/brk-client/index.js:978](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L978)

Iterate over [index, value] pairs

#### Returns

`IterableIterator`\<\[`number`, `T`\]\>

***

### iterDates()

> **iterDates**: () => `IterableIterator`\<\[`Date`, `T`\]\>

Defined in: [Developer/brk/modules/brk-client/index.js:979](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L979)

Iterate over [date, value] pairs (date-based only)

#### Returns

`IterableIterator`\<\[`Date`, `T`\]\>

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:970](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L970)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:968](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L968)

Start index (inclusive)

***

### toDateMap()

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:974](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L974)

Return data as Map keyed by date (date-based only)

#### Returns

`Map`\<`Date`, `T`\>

***

### toIndexMap()

> **toIndexMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:975](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L975)

Return data as Map keyed by index

#### Returns

`Map`\<`number`, `T`\>

***

### total

> **total**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:967](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L967)

Total number of data points

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:965](https://github.com/bitcoinresearchkit/brk/blob/36bc1fb4912b070acbe47217b1be5b7c1096f80f/modules/brk-client/index.js#L965)

Version of the metric data
