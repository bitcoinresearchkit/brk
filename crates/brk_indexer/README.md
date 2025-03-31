# BRK Indexer

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
  <a href="https://crates.io/crates/brk_indexer">
    <img src="https://img.shields.io/crates/v/brk_indexer" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_indexer">
    <img src="https://img.shields.io/docsrs/brk_indexer" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_indexer" alt="Size" />
  <a href="https://deps.rs/crate/brk_indexer">
    <img src="https://deps.rs/crate/brk_indexer/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/Cvrwpv3zEG">
    <img src="https://img.shields.io/discord/1350431684562124850" alt="Chat" />
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

A [Bitcoin Core](https://bitcoincore.org/en/about/) node indexer which iterates over the chain (via `../brk_parser`) and creates a database of the vecs (`brk_vec`) and key/value stores ([`fjall`](https://crates.io/crates/fjall)) that can be used in your Rust code.

The crate only stores the bare minimum to be self sufficient and not have to use an RPC client (except for scripts which are not stored). If you need more data, checkout `../computer` which uses the outputs from the indexer to compute a whole range of datasets.

Vecs are used sparingly instead of stores for multiple reasons:

- Only stores the relevant data since the key is an index
- Saved as uncompressed bytes and thus can be parsed manually (with any programming language) without relying on a server or library
- Easy to work with and predictable

## Usage

Storage wise, the expected overhead should be around 30% of the chain itself.

Peaks at 5 GB of RAM

## Outputs

Vecs: `src/storage/vecs/mod.rs`

Stores: `src/storage/stores/mod.rs`

## Benchmark

Indexing `0..885_835` took `11 hours 6 min 50 s` on a Macbook Pro M3 Pro with 36 GB of RAM

`footprint` report:
- Peak memory: `5115 MB`
- Memory while waiting for a new block: `890 MB`
- Reclaimable memory: `6478 MB`
