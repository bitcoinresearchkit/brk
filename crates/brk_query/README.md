# BRK Query

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
  <a href="https://crates.io/crates/brk_query">
    <img src="https://img.shields.io/crates/v/brk_query" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_query">
    <img src="https://img.shields.io/docsrs/brk_query" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_query" alt="Size" />
  <a href="https://deps.rs/crate/brk_query">
    <img src="https://deps.rs/crate/brk_query/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/Cvrwpv3zEG">
    <img src="https://img.shields.io/discord/1350431684562124850" alt="Chat" />
  </a>
</p>

A crate that searches for datasets from either `brk_indexer` or `brk_computer` according to given parameters.

It's possible to search for one or multiple dataset if they have the same index and specify range with both the `from` and `to` being optional and supporting negative values.

The output will depend on the format choson which can be Markdown, Json, CSV or TSV and might vary if there is a one or mutiple datasets, and if one dataset one or multiple values.

In the future, it will support more features similar to a real query engine like in a Postgres databases and presets to fetch data grouped by address, transaction or blockhash/height.
