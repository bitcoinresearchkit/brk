[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MetricData

# Interface: MetricData\<T\>

Defined in: [Developer/brk/modules/brk-client/index.js:939](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L939)

## Type Parameters

### T

`T`

## Properties

### data

> **data**: `T`[]

Defined in: [Developer/brk/modules/brk-client/index.js:946](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L946)

The metric data

***

### dateEntries()

> **dateEntries**: () => \[`Date`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:951](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L951)

Return data as [date, value] pairs (date-based only)

#### Returns

\[`Date`, `T`\][]

***

### dates()

> **dates**: () => `Date`[]

Defined in: [Developer/brk/modules/brk-client/index.js:947](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L947)

Convert index range to dates (date-based indexes only)

#### Returns

`Date`[]

***

### end

> **end**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:944](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L944)

End index (exclusive)

***

### index

> **index**: [`Index`](../type-aliases/Index.md)

Defined in: [Developer/brk/modules/brk-client/index.js:941](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L941)

The index type used for this query

***

### indexEntries()

> **indexEntries**: () => \[`number`, `T`\][]

Defined in: [Developer/brk/modules/brk-client/index.js:952](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L952)

Return data as [index, value] pairs

#### Returns

\[`number`, `T`\][]

***

### indexes()

> **indexes**: () => `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:948](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L948)

Get index range as array

#### Returns

`number`[]

***

### iter()

> **iter**: () => `IterableIterator`\<\[`number`, `T`\]\>

Defined in: [Developer/brk/modules/brk-client/index.js:953](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L953)

Iterate over [index, value] pairs

#### Returns

`IterableIterator`\<\[`number`, `T`\]\>

***

### iterDates()

> **iterDates**: () => `IterableIterator`\<\[`Date`, `T`\]\>

Defined in: [Developer/brk/modules/brk-client/index.js:954](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L954)

Iterate over [date, value] pairs (date-based only)

#### Returns

`IterableIterator`\<\[`Date`, `T`\]\>

***

### stamp

> **stamp**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:945](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L945)

ISO 8601 timestamp of when the response was generated

***

### start

> **start**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:943](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L943)

Start index (inclusive)

***

### toDateMap()

> **toDateMap**: () => `Map`\<`Date`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:949](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L949)

Return data as Map keyed by date (date-based only)

#### Returns

`Map`\<`Date`, `T`\>

***

### toIndexMap()

> **toIndexMap**: () => `Map`\<`number`, `T`\>

Defined in: [Developer/brk/modules/brk-client/index.js:950](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L950)

Return data as Map keyed by index

#### Returns

`Map`\<`number`, `T`\>

***

### total

> **total**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:942](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L942)

Total number of data points

***

### version

> **version**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:940](https://github.com/bitcoinresearchkit/brk/blob/54827cd0a2357417adb8631eb8f53a78e848b39a/modules/brk-client/index.js#L940)

Version of the metric data
