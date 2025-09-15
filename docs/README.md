# Bitcoin Research Kit

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk">
    <img src="https://img.shields.io/crates/v/brk" alt="Version" />
  </a>
  <a href="https://docs.rs/brk">
    <img src="https://img.shields.io/docsrs/brk" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk" alt="Size" />
  <a href="https://deps.rs/crate/brk">
    <img src="https://deps.rs/crate/brk/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/WACpShCB7M">
    <img src="https://img.shields.io/discord/1350431684562124850?label=discord" alt="Discord" />
  </a>
  <a href="https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6">
    <img src="https://img.shields.io/badge/nostr-purple?link=https%3A%2F%2Fprimal.net%2Fp%2Fnprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6" alt="Nostr" />
  </a>
  <a href="https://opensats.org">
    <img src="https://img.shields.io/badge/%3E__-opensats-rgb(249,115,22)" alt="opensats" />
  </a>
</p>

The Bitcoin Research Kit is a high-performance toolchain designed to parse, index, compute, serve and visualize data from a Bitcoin node, enabling users to gain deeper insights into the Bitcoin network.

In other words it's an alternative to [Glassnode](https://glassnode.com), [mempool.space](https://mempool.space/) (soon) and [electrs](https://github.com/romanz/electrs) (soon) all in one package with a particular focus on simplicity and ease of use.

The toolkit can be used in various ways to accommodate as many needs as possible:

- **[Website](https://bitview.space)** \
  Everyone is welcome to visit the official instance and showcase of the suite's capabilities. \
  It has a wide range of functionalities including charts, tables and simulations which you can visit for free and without the need for an account.
- **[API](https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_server#brk-server)** \
  Researchers and developers are free to use BRK's public API with ![Datasets variant count](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fbitcoinresearchkit.org%2Fapi%2Fvecs%2Fvec-count&query=%24&style=flat&label=%20&color=white) dataset variants at their disposal. \
  Just like the website, it's entirely free, with no authentication or rate-limiting.
- **[AI](https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_mcp/README.md#brk-mcp)** \
  LLMs have to possibility to connect to BRK's backend through a [MCP](https://modelcontextprotocol.io/introduction). \
  It will give them access to the same tools as the API, with no restrictions, and allow you to have your very own data analysts.
- **[CLI](https://crates.io/crates/brk_cli)** \
  Node runners are strongly encouraged to try out and self-host their own instance using BRK's command line interface. \
  The CLI has multiple cogs available for users to tweak to adapt to all situations with even the possibility for web developers to create their own custom website which could later on be added as an alternative front-end.
- **[Crates](https://crates.io/crates/brk)** \
  Rust developers have access to a wide range crates, each built upon one another with its own specific purpose, enabling independent use and offering great flexibility.
  PRs are welcome, especially if their goal is to introduce additional datasets.

The primary goal of this project is to be fully-featured and accessible for everyone, regardless of their background or financial situation - whether that person is an enthusiast, researcher, miner, analyst, or simply curious.

In contrast, existing alternatives tend to be either [very costly](https://studio.glassnode.com/pricing) or missing essential features, with the vast majority being closed-source and unverifiable, which fundamentally undermines the principles of Bitcoin.

## Hosting as a service

If you'd like to have your own instance hosted for you please contact [hosting@bitcoinresearchkit.org](mailto:hosting@bitcoinresearchkit.org).

- 2 separate dedicated servers (1 GB/s each) with different ISPs and Cloudflare integration for enhanced performance and optimal availability
- 99.99% SLA
- Configured for speed
- Updates delivered at your convenience
- Direct communication for feature requests and support
- Bitcoin Core or Knots with desired version
- Optional subdomains
- Logo featured in the Readme if desired

Pricing: `0.01 BTC / month` *or* `0.1 BTC / year`

## Acknowledgments

Deepest gratitude to the [Open Sats](https://opensats.org/) public charity. Their grant — from December 2024 to the present — has been critical in sustaining this project.

Heartfelt thanks go out to every donor on [Nostr](https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44) and [Geyser.fund](https://geyser.fund/project/brk) whose support has ensured the availability of the [bitcoinresearchkit.org](https://bitcoinresearchkit.org) public instance.

## Crates

- [`brk`](https://crates.io/crates/brk): A wrapper around all other `brk-*` crates
- [`brk_bundler`](https://crates.io/crates/brk_bundler): A thin wrapper around [`rolldown`](https://rolldown.rs/)
- [`brk_cli`](https://crates.io/crates/brk_cli): A command line interface to run a BRK instance
- [`brk_computer`](https://crates.io/crates/brk_computer): A Bitcoin dataset computer built on top of [`brk_indexer`](https://crates.io/crates/brk_indexer)
- [`brk_error`](https://crates.io/crates/brk_error): Errors used throughout BRK
- [`brk_fetcher`](https://crates.io/crates/brk_fetcher): A Bitcoin price fetcher
- [`brk_indexer`](https://crates.io/crates/brk_indexer): A Bitcoin indexer built on top of [`brk_parser`](https://crates.io/crates/brk_parser)
- [`brk_interface`](https://crates.io/crates/brk_interface): An interface to find and format data from BRK
- [`brk_logger`](https://crates.io/crates/brk_logger): A thin wrapper around [`env_logger`](https://crates.io/crates/env_logger)
- [`brk_mcp`](https://crates.io/crates/brk_mcp): A bridge for LLMs to access BRK
- [`brk_parser`](https://crates.io/crates/brk_parser): A very fast Bitcoin block parser and iterator built on top of [`bitcoin-rust`](https://crates.io/crates/bitcoin)
- [`brk_server`](https://crates.io/crates/brk_server): A server with an API for anything from BRK
- [`brk_store`](https://crates.io/crates/brk_store): A thin wrapper around [`fjall`](https://crates.io/crates/fjall)
- [`brk_structs`](https://crates.io/crates/brk_structs): Structs used throughout BRK

## Donate

[`bc1q09 8zsm89 m7kgyz e338vf ejhpdt 92ua9p 3peuve`](bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve)
