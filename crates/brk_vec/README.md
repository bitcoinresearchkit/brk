# BRK Vec

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk_vec">
    <img src="https://img.shields.io/crates/v/brk_vec" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_vec">
    <img src="https://img.shields.io/docsrs/brk_vec" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_vec" alt="Size" />
  <a href="https://deps.rs/crate/brk_vec">
    <img src="https://deps.rs/crate/brk_vec/latest/status.svg" alt="Dependency status">
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

A `Vec` (an array) that is stored on disk and thus which can be much larger than the available RAM.

Compared to a key/value store, the data stored is raw byte interpretation of the Vec's values without any overhead which is very efficient. Additionally it uses close to no RAM when caching isn't active and up to 100 MB when it is.

Compression is also available and built on top [`zstd`](https://crates.io/crates/zstd) to save even more space (from 0 to 75%). The tradeoff being slower reading speeds, especially random reading speeds. This is due to the data being stored in compressed pages of 16 KB, which means that if you to read even one value in that page you have to uncompress the whole page.
