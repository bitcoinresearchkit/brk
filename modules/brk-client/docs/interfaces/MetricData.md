[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricData

# Interface: MetricData\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:1002](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1002)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1009](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1009)

The metric data

***

### dateEntries()

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1014](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1014)

Return data as [date, value] pairs (date-based only)

#### Returns

\[`Date`, `T`\][]

***

### dates()

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1010](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1010)

Convert index range to dates (date-based indexes only)

#### Returns

`Date`[]

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1007](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1007)

End index (exclusive)

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:1004](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1004)

The index type used for this query

***

### indexEntries()

> **indexEntries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:1015](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1015)

Return data as [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### indexes()

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:1011](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1011)

Get index range as array

#### Returns

`number`[]

***

### iter()

> **iter**: () => `IterableIterator`\<\[`number`, `T`\]\>

Defined in: [Developer/brk/modules/brk-client/index.js:1016](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1016)

Iterate over [index, value] pairs

#### Returns

`IterableIterator`\<\[`number`, `T`\]\>

***

### iterDates()

> **iterDates**: () => `IterableIterator`\<\[`Date`, `T`\]\>

Defined in: [Developer/brk/modules/brk-client/index.js:1017](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1017)

Iterate over [date, value] pairs (date-based only)

#### Returns

`IterableIterator`\<\[`Date`, `T`\]\>

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:1008](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1008)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1006](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1006)

Start index (inclusive)

***

### toDateMap()

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1012](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1012)

Return data as Map keyed by date (date-based only)

#### Returns

`Map`\<`Date`, `T`\>

***

### toIndexMap()

> **toIndexMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:1013](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1013)

Return data as Map keyed by index

#### Returns

`Map`\<`number`, `T`\>

***

### total

> **total**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1005](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1005)

Total number of data points

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:1003](https://github.com/bitcoinresearchkit/brk/blob/dfcb04484b3a4203772bb13ff5945fb24d8bbd9f/modules/brk-client/index.js#L1003)

Version of the metric data
