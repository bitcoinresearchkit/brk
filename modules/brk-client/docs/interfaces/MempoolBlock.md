[**brk-client**](../README.md)

***

[brk-client](../globals.md) / MempoolBlock

# Interface: MempoolBlock

Defined in: [Developer/brk/modules/brk-client/index.js:597](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L597)

## Properties

### blockSize

> **blockSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:598](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L598)

Total block size in weight units

***

### blockVSize

> **blockVSize**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:599](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L599)

Total block virtual size in vbytes

***

### feeRange

> **feeRange**: `number`[]

Defined in: [Developer/brk/modules/brk-client/index.js:603](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L603)

Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]

***

### medianFee

> **medianFee**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:602](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L602)

Median fee rate in sat/vB

***

### nTx

> **nTx**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:600](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L600)

Number of transactions in the projected block

***

### totalFees

> **totalFees**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:601](https://github.com/bitcoinresearchkit/brk/blob/41ec24c81e5075cce1fd92557af02306ad5e61c7/modules/brk-client/index.js#L601)

Total fees in satoshis
