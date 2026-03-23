[**brk-client**](../README.md)

***

[brk-client](../globals.md) / FundedAddrData

# Interface: FundedAddrData

Defined in: [Developer/brk/modules/brk-client/index.js:373](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L373)

## Properties

### fundedTxoCount

> **fundedTxoCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:375](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L375)

Number of transaction outputs funded to this address

***

### investorCapRaw

> **investorCapRaw**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:380](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L380)

The investor capitalization: Σ(price² × sats)

***

### realizedCapRaw

> **realizedCapRaw**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:379](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L379)

The realized capitalization: Σ(price × sats)

***

### received

> **received**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:377](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L377)

Satoshis received by this address

***

### sent

> **sent**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:378](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L378)

Satoshis sent by this address

***

### spentTxoCount

> **spentTxoCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:376](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L376)

Number of transaction outputs spent by this address

***

### txCount

> **txCount**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:374](https://github.com/bitcoinresearchkit/brk/blob/14d7adfdd525d3ec62b22ccc6dc641552e0d5ef0/modules/brk-client/index.js#L374)

Total transaction count
