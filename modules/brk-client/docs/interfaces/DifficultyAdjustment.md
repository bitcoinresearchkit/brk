[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:251](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L251)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:260](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L260)

Time-adjusted average (accounting for timestamp manipulation)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:253](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L253)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:254](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L254)

Estimated Unix timestamp of next retarget

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:258](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L258)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:257](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L257)

Previous difficulty adjustment (%)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:252](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L252)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:255](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L255)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:256](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L256)

Estimated seconds until retarget

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:259](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L259)

Average block time in current epoch (seconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:261](https://github.com/bitcoinresearchkit/brk/blob/ba60b7e4f64e81cffbd4781566a0df0728014881/modules/brk-client/index.js#L261)

Time offset from expected schedule (seconds)
