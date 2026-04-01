[**brk-client**](../README.md)

***

[brk-client](../globals.md) / DifficultyAdjustment

# Interface: DifficultyAdjustment

Defined in: [Developer/brk/modules/brk-client/index.js:386](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L386)

## Properties

### adjustedTimeAvg

> **adjustedTimeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:395](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L395)

Time-adjusted average (accounting for timestamp manipulation)

***

### difficultyChange

> **difficultyChange**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:388](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L388)

Estimated difficulty change at next retarget (%)

***

### estimatedRetargetDate

> **estimatedRetargetDate**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:389](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L389)

Estimated Unix timestamp of next retarget

***

### nextRetargetHeight

> **nextRetargetHeight**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:393](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L393)

Height of next retarget

***

### previousRetarget

> **previousRetarget**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:392](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L392)

Previous difficulty adjustment (%)

***

### progressPercent

> **progressPercent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:387](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L387)

Progress through current difficulty epoch (0-100%)

***

### remainingBlocks

> **remainingBlocks**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:390](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L390)

Blocks remaining until retarget

***

### remainingTime

> **remainingTime**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:391](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L391)

Estimated seconds until retarget

***

### timeAvg

> **timeAvg**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:394](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L394)

Average block time in current epoch (seconds)

***

### timeOffset

> **timeOffset**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:396](https://github.com/bitcoinresearchkit/brk/blob/83edef4806773ef7225b7c0de863c44bd953169d/modules/brk-client/index.js#L396)

Time offset from expected schedule (seconds)
