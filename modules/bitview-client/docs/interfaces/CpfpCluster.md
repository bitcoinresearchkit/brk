[**bitview-client**](../README.md)

***

[bitview-client](../globals.md) / CpfpCluster

# Interface: CpfpCluster

Defined in: [Developer/mono/modules/bitview-client/index.js:430](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L430)

## Properties

### chunkIndex

> **chunkIndex**: `number`

Defined in: [Developer/mono/modules/bitview-client/index.js:433](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L433)

Index into `chunks` of the chunk containing the seed tx.

***

### chunks

> **chunks**: [`CpfpClusterChunk`](CpfpClusterChunk.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:432](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L432)

SFL-emitted chunks ordered by descending feerate.

***

### txs

> **txs**: [`CpfpClusterTx`](CpfpClusterTx.md)[]

Defined in: [Developer/mono/modules/bitview-client/index.js:431](https://github.com/bitcoinresearchkit/brk/blob/0f15a1b7c568a2f7b912d2fccd42fd2565d5f337/modules/bitview-client/index.js#L431)

All txs in the cluster, in topological order (parents before children).
