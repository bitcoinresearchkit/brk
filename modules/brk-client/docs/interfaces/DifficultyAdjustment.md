[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:524](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L524)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:534](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L534)

Time-adjusted average (milliseconds)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:526](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L526)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:527](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L527)

Estimated timestamp of next retarget (milliseconds)

***

### expectedBlocks

> **expectedBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:536](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L536)

Expected blocks based on wall clock time since epoch start

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:532](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L532)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:530](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L530)

Previous difficulty adjustment (%)

***

### previousTime

> **previousTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:531](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L531)

Timestamp of most recent retarget (seconds)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:525](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L525)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:528](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L528)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:529](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L529)

Estimated time until retarget (milliseconds)

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:533](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L533)

Average block time in current epoch (milliseconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:535](https://github.com/bitcoinresearchkit/brk/blob/7a718293c0ddbae305c8352474c81c0e99fe1200/modules/brk-client/index.js#L535)

Time offset from expected schedule (seconds)
