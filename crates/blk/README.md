# blk

A CLI to inspect Bitcoin Core blocks.

Reads `blk*.dat` files directly via [`brk_reader`](../brk_reader) and resolves
the chain tip / heights via the Bitcoin Core RPC. Output is shell-friendly:
bare values, NDJSON, pretty JSON, or TSV.

## Install

```sh
cargo install --path crates/blk
```

## Quick start

```sh
blk 800000 hash                        # bare hash
blk 800000 height hash time            # one compact JSON line
blk 800000 tx.0.vout.0.value           # coinbase output 0 sats
blk 0..2 hash tx.0.txid                # 3 NDJSON lines
blk tip tx.0                           # whole coinbase tx as JSON
```

## Reference

Run `blk --help` for the full field/selector/option reference.
