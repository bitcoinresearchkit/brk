[**brk-client**](../README.md)

***

[brk-client](../globals.md) / CpfpCluster

# Interface: CpfpCluster

Defined in: [Developer/brk/modules/brk-client/index.js:427](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L427)

## Properties

### chunkIndex

> **chunkIndex**: `number`

Defined in: [Developer/brk/modules/brk-client/index.js:430](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L430)

Index into `chunks` of the chunk containing the seed tx.

***

### chunks

> **chunks**: [`CpfpClusterChunk`](CpfpClusterChunk.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:429](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L429)

SFL-emitted chunks ordered by descending feerate.

***

### txs

> **txs**: [`CpfpClusterTx`](CpfpClusterTx.md)[]

Defined in: [Developer/brk/modules/brk-client/index.js:428](https://github.com/bitcoinresearchkit/brk/blob/0b871e86004ed9dd0c54dd9336049531d6fe4d23/modules/brk-client/index.js#L428)

All txs in the cluster, in topological order (parents before children).
