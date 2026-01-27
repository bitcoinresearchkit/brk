[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:195](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L195)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:204](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L204)

Time-adjusted average (accounting for timestamp manipulation)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:197](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L197)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:198](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L198)

Estimated Unix timestamp of next retarget

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:202](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L202)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:201](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L201)

Previous difficulty adjustment (%)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:196](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L196)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:199](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L199)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:200](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L200)

Estimated seconds until retarget

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:203](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L203)

Average block time in current epoch (seconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:205](https://github.com/bitcoinresearchkit/brk/blob/3d3652470714070e3e6df47b3af6f06512541475/modules/brk-client/index.js#L205)

Time offset from expected schedule (seconds)
