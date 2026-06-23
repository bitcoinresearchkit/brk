[**brk-client**](../README.md)

***

[brk-client](../globals.md) / AddrStats

# Interface: AddrStats

Defined in: [Developer/brk/modules/brk-client/index.js:65](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L65)

## Properties

### address

> **address**: `string`

Defined in: [Developer/brk/modules/brk-client/index.js:66](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L66)

Bitcoin address string

***

### addrType

> **addrType**: [`OutputType`](../type-aliases/OutputType.md)

Defined in: [Developer/brk/modules/brk-client/index.js:67](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L67)

Address type (p2pkh, p2sh, v0_p2wpkh, v0_p2wsh, v1_p2tr, etc.)

***

### chainStats

> **chainStats**: [`AddrChainStats`](AddrChainStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:68](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L68)

Statistics for confirmed transactions on the blockchain

***

### mempoolStats

> **mempoolStats**: [`AddrMempoolStats`](AddrMempoolStats.md)

Defined in: [Developer/brk/modules/brk-client/index.js:69](https://github.com/bitcoinresearchkit/brk/blob/9879a986aa1b6609c39bacbd2ccae5519598e212/modules/brk-client/index.js#L69)

Statistics for unconfirmed transactions in the mempool
