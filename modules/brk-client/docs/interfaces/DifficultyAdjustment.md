[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:392](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L392)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:402](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L402)

Time-adjusted average (milliseconds)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:394](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L394)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:395](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L395)

Estimated timestamp of next retarget (milliseconds)

***

### expectedBlocks

> **expectedBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:404](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L404)

Expected blocks based on wall clock time since epoch start

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:400](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L400)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:398](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L398)

Previous difficulty adjustment (%)

***

### previousTime

> **previousTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:399](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L399)

Timestamp of most recent retarget (seconds)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:393](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L393)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:396](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L396)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:397](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L397)

Estimated time until retarget (milliseconds)

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:401](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L401)

Average block time in current epoch (milliseconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:403](https://github.com/bitcoinresearchkit/brk/blob/6cd45c1f1f755807c6e6ab6ba61b54aff4b07f3b/modules/brk-client/index.js#L403)

Time offset from expected schedule (seconds)
