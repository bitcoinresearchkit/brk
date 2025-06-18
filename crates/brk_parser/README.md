# BRK Parser

This Rust library reads raw block files (like `blkXXXXX.dat`) from Bitcoin Core data, and creates an iterator over the 
requested range of blocks in sequential order.

The element returned by the iterator is a tuple consisting of:
- the block `Height`
- the `Block` (from `bitcoin-rust`), and
- the block's `BlockHash` (also from `bitcoin-rust`)

Tested with Bitcoin Core `v25.0..=v28.1`

## Requirements

Even though this parser reads `blkXXXXX.dat` files directly, it **needs** `bitcoind` running to provide the RPC Server 
to filter out blockchain tip-forks.

Peak RAM required should be around 500 MB.

XOR-ed blocks are supported.

## Disclaimer

> [!IMPORTANT]  
> A state of the local chain is saved in `{bitcoindir}/blocks/blk_index_to_blk_recap.json` to allow for faster starts (see benchmark below) but presently doesn't support locking. Therfore you should run only one instance of `brk_parser` at a time.

## Benchmark

|  | [brk_parser](https://crates.io/crates/brk_parser) | [bitcoin-explorer (deprecated)](https://crates.io/crates/bitcoin-explorer) | [blocks_iterator](https://crates.io/crates/blocks_iterator)* |
| --- | --- | --- | --- |
| Runs **with** `bitcoind` | Yes ✅ | No ❌ | Yes ✅ |
| Runs **without** `bitcoind` | No ❌ | Yes ✅ | Yes ✅ |
| `0..=855_000` | 4mn 10s | 4mn 45s | > 2h |
| `800_000..=855_000` | 0mn 52s (4mn 10s if first run) | 0mn 55s | > 2h |

\* `blocks_iterator` is with the default config (and thus with `skip_prevout = false` which does 
much more than just iterate over blocks) so this isn't strictly an apples to apples comparison.  
So the results are somewhat misleading. You should expect much closer times. Will update the 
benchmark with `skip_prevout = true` as soon as possible.

*Benchmarked on a Macbook Pro M3 Pro*

----
<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk_cli">
    <img src="https://img.shields.io/crates/v/brk_cli" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_cli">
    <img src="https://img.shields.io/docsrs/brk_cli" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_cli" alt="Size" />
  <a href="https://deps.rs/crate/brk_cli">
    <img src="https://deps.rs/crate/brk_cli/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/HaR3wpH3nr">
    <img src="https://img.shields.io/discord/1350431684562124850?label=discord" alt="Discord" />
  </a>
  <a href="https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6">
    <img src="https://img.shields.io/badge/nostr-purple?link=https%3A%2F%2Fprimal.net%2Fp%2Fnprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6" alt="Nostr" />
  </a>
  <a href="https://bsky.app/profile/bitcoinresearchkit.org">
    <img src="https://img.shields.io/badge/bluesky-blue?link=https%3A%2F%2Fbsky.app%2Fprofile%2Fbitcoinresearchkit.org" alt="Bluesky" />
  </a>
  <a href="https://x.com/brkdotorg">
    <img src="https://img.shields.io/badge/x.com-black" alt="X" />
  </a>
</p>
