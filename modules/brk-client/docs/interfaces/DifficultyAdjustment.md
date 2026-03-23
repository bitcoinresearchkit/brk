[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:293](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L293)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:302](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L302)

Time-adjusted average (accounting for timestamp manipulation)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:295](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L295)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:296](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L296)

Estimated Unix timestamp of next retarget

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:300](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L300)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:299](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L299)

Previous difficulty adjustment (%)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:294](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L294)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:297](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L297)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:298](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L298)

Estimated seconds until retarget

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:301](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L301)

Average block time in current epoch (seconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:303](https://github.com/bitcoinresearchkit/brk/blob/ec6420254a230ae65df0ed8b66cad1cffcf46447/modules/brk-client/index.js#L303)

Time offset from expected schedule (seconds)
