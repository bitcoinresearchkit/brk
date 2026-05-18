[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:512](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L512)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:522](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L522)

Time-adjusted average (milliseconds)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:514](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L514)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:515](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L515)

Estimated timestamp of next retarget (milliseconds)

***

### expectedBlocks

> **expectedBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:524](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L524)

Expected blocks based on wall clock time since epoch start

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:520](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L520)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:518](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L518)

Previous difficulty adjustment (%)

***

### previousTime

> **previousTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:519](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L519)

Timestamp of most recent retarget (seconds)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:513](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L513)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:516](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L516)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:517](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L517)

Estimated time until retarget (milliseconds)

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:521](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L521)

Average block time in current epoch (milliseconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:523](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L523)

Time offset from expected schedule (seconds)
