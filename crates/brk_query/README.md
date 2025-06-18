# BRK Query

A crate that searches for datasets from either `brk_indexer` or `brk_computer` according to given 
parameters.

It's possible to search for one or multiple dataset if they have the same index and specify range 
with both the `from` and `to` being optional and supporting negative values.

The output will depend on the format chosen which can be Markdown, Json, CSV or TSV and might vary 
if there is a one or multiple datasets, and if one dataset one or multiple values.

> [!NOTE]  
>In the future, BRK Query will gradually acquire features such as: SQL queries and the ability to fetch data by address, transaction ID, block hash, or block height, among others.

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
