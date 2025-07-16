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

The Bitcoin Research Kit is a high-performance toolchain designed to parse, index, compute, serve and visualize data from a Bitcoin node, enabling users to gain deeper insights into the Bitcoin network.

In other words it's an alternative to [Glassnode](https://glassnode.com), [mempool.space](https://mempool.space/) (soon) and [electrs](https://github.com/romanz/electrs) (soon) all in one package with a particular focus on simplicity and ease of use.

The toolkit can be used in various ways to accommodate as many needs as possible:

- **[Website](https://bitcoinresearchkit.org)** \
  Everyone is welcome to visit the official instance and showcase of the suite's capabilities. \
  It has a wide range of functionalities including charts, tables and simulations which you can visit for free and without the need for an account. \
  Also available at: [brekit.org](https://brekit.org) // [kibo.money](https://kibo.money) // [satonomics.xyz](https://satonomics.xyz)
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

## Crates

- [`brk`](https://crates.io/crates/brk): Wrapper around all other `brk-*` crates
- [`brk_cli`](https://crates.io/crates/brk_cli): A command line interface to run a Bitcoin Research Kit instance
- [`brk_computer`](https://crates.io/crates/brk_computer): A Bitcoin dataset computer, built on top of brk_indexer
- [`brk_core`](https://crates.io/crates/brk_core): The Core (Structs and Errors) of the Bitcoin Research Kit
- [`brk_exit`](https://crates.io/crates/brk_exit): An exit blocker built on top of ctrlc
- [`brk_fetcher`](https://crates.io/crates/brk_fetcher): A Bitcoin price fetcher
- [`brk_indexer`](https://crates.io/crates/brk_indexer): A Bitcoin Core indexer built on top of brk_parser
- [`brk_logger`](https://crates.io/crates/brk_logger): A clean logger used in the Bitcoin Research Kit
- [`brk_mcp`](https://crates.io/crates/brk_mcp): A Model Context Protocol (MCP) which gives LLMs access to all available tools in BRK
- [`brk_parser`](https://crates.io/crates/brk_parser): A very fast Bitcoin Core block parser and iterator built on top of bitcoin-rust
- [`brk_interface`](https://crates.io/crates/brk_interface): An interface to BRK's engine
- [`brk_server`](https://crates.io/crates/brk_server): A server that serves Bitcoin data and swappable front-ends, built on top of `brk_indexer`, `brk_fetcher` and `brk_computer`
- [`brk_store`](https://crates.io/crates/brk_store): A thin wrapper around [`fjall`](https://crates.io/crates/fjall)
- [`brk_vec`](https://crates.io/crates/brk_vec): A storeable vec
- [`brk_bundler`](https://crates.io/crates/brk_bundler): A thin wrapper around [`rolldown`](https://rolldown.rs/)

## Hosting as a service

If you'd like to have your own instance hosted for you please contact [hosting@bitcoinresearchkit.org](mailto:hosting@bitcoinresearchkit.org).

- 2 separate dedicated servers (1 GB/s each) with different ISPs and Cloudflare integration for enhanced performance and optimal availability
- 99.99% SLA
- Configured for speed
- Updates delivered at your convenience
- Direct communication for feature requests and support
- Bitcoin Core or Knots with desired version
- Optional subdomains: `*.bitcoinresearchkit.org`, `*.kibo.money` and `*.satonomics.xyz`
- Logo featured in the Readme if desired

Pricing: `0.01 BTC / month` *or* `0.1 BTC / year`

## Acknowledgments

Deepest gratitude to the [Open Sats](https://opensats.org/) public charity. Their grant — from December 2024 to the present — has been critical in sustaining this project.

Heartfelt thanks go out to every donor on [Nostr](https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44) and [Geyser.fund](https://geyser.fund/project/brk) whose support has ensured the availability of the [bitcoinresearchkit.org](https://bitcoinresearchkit.org) public instance.

## Donate

[`bc1q09 8zsm89 m7kgyz e338vf ejhpdt 92ua9p 3peuve`](bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve)

[`lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4`](lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4)

[Geyser Fund](https://geyser.fund/project/brk)
