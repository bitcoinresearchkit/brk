[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:464](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L464)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:474](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L474)

Time-adjusted average (milliseconds)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:466](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L466)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:467](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L467)

Estimated timestamp of next retarget (milliseconds)

***

### expectedBlocks

> **expectedBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:476](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L476)

Expected blocks based on wall clock time since epoch start

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:472](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L472)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:470](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L470)

Previous difficulty adjustment (%)

***

### previousTime

> **previousTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:471](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L471)

Timestamp of most recent retarget (seconds)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:465](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L465)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:468](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L468)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:469](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L469)

Estimated time until retarget (milliseconds)

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:473](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L473)

Average block time in current epoch (milliseconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:475](https://github.com/bitcoinresearchkit/brk/blob/25b226856307047c5c10a075ec57219ca9987c38/modules/brk-client/index.js#L475)

Time offset from expected schedule (seconds)
