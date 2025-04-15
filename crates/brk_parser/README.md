# BRK Parser

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://kibo.money">
    <img alt="kibo.money" src="https://img.shields.io/badge/showcase-kib%C5%8D.money-orange">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk_parser">
    <img src="https://img.shields.io/crates/v/brk_parser" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_parser">
    <img src="https://img.shields.io/docsrs/brk_parser" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_parser" alt="Size" />
  <a href="https://deps.rs/crate/brk_parser">
    <img src="https://deps.rs/crate/brk_parser/latest/status.svg" alt="Dependency status">
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
  <a href="https://x.com/0xbrk">
    <img src="https://img.shields.io/badge/x.com-black" alt="X" />
  </a>
</p>

A very fast and simple Rust library which reads raw block files (*blkXXXXX.dat*) from Bitcoin Core node and creates an iterator over all the requested blocks in sequential order (0, 1, 2, ...).

The element returned by the iterator is a tuple which includes the:
- Height: `Height`
- Block: `Block` (from `bitcoin-rust`)
- Block's Hash: `BlockHash` (also from `bitcoin-rust`)

## Requirements

Even though it reads *blkXXXXX.dat* files, it **needs** `bitcoind` to run with the RPC server to filter out block forks.

Peak memory should be around 500MB.

## Disclaimer

A state is saved in `{bitcoindir}/blocks/blk_index_to_blk_recap.json` to allow for faster starts (see benchmark below) but doesn't yet support locking. Thus it is recommended to run one instance of `brk_parser` at a time.

## Comparaison

|  | [brk_parser](https://crates.io/crates/brk_parser) | [bitcoin-explorer (deprecated)](https://crates.io/crates/bitcoin-explorer) | [blocks_iterator](https://crates.io/crates/blocks_iterator) |
| --- | --- | --- | --- |
| Runs **with** `bitcoind` | Yes ✅ | No ❌ | Yes ✅ |
| Runs **without** `bitcoind` | No ❌ | Yes ✅ | Yes ✅ |
| `0..=855_000` | 4mn 10s | 4mn 45s | > 2h |
| `800_000..=855_000` | 0mn 52s (4mn 10s if first run) | 0mn 55s | > 2h |

*Benchmarked on a Macbook Pro M3 Pro*
