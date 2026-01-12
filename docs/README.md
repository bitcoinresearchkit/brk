# Bitcoin Research Kit

**Open-source Bitcoin analytics infrastructure.**

[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/brk.svg)](https://crates.io/crates/brk)
[![docs.rs](https://img.shields.io/docsrs/brk)](https://docs.rs/brk)
[![Discord](https://img.shields.io/discord/1350431684562124850?logo=discord)](https://discord.gg/WACpShCB7M)

[Homepage](https://bitcoinresearchkit.org) · [**Bitview**](https://bitview.space) · [API Reference](https://bitcoinresearchkit.org/api)

---

BRK parses, indexes, and analyzes Bitcoin blockchain data. It combines on-chain analytics (like [Glassnode](https://glassnode.com)), block exploration (like [mempool.space](https://mempool.space)), and address indexing (like [electrs](https://github.com/romanz/electrs)) into a single self-hostable package.

## See It In Action

[**Bitview**](https://bitview.space) is a web application built entirely on BRK. It offers interactive charts for exploring Bitcoin on-chain metrics—price models, supply dynamics, holder behavior, network activity, and more. Browse it to see what's possible with the data BRK provides.

## What It Provides

**On-Chain Metrics** — Thousands of derived metrics: market indicators (realized cap, MVRV, SOPR, NVT), supply analysis (circulating, liquid, illiquid), holder cohorts (by balance, age, address type), and pricing models. This is what sets BRK apart from typical block explorers.

**Blockchain Data** — Blocks, transactions, addresses, UTXOs. The API follows mempool.space's format for compatibility with existing tools.

**Multiple Indexes** — Query data by date, block height, halving epoch, address type, UTXO age, and more. Enables flexible time-series queries and cohort analysis.

**Mempool** — Real-time fee estimation, projected blocks, unconfirmed transaction tracking.

**REST API** — JSON and CSV output with OpenAPI documentation.

**MCP Server** — Model Context Protocol integration for AI assistants and LLMs.

## Get Started

**Use the Public API** — Access data without running infrastructure. Client libraries available for [JavaScript](https://www.npmjs.com/package/brk-client), [Python](https://pypi.org/project/brk-client/), and [Rust](https://crates.io/crates/brk_client). See the [API reference](https://bitcoinresearchkit.org/api) for endpoints.

**Self-Host** — Run your own instance with Bitcoin Core. Install via `cargo install --locked brk_cli` or use [Docker](https://github.com/bitcoinresearchkit/brk/tree/main/docker). Requires ~400 GB disk and 12+ GB RAM. See the [hosting guide](./HOSTING.md).

**Use as a Library** — Build custom tools with the Rust crates. Use individual components or the [umbrella crate](https://docs.rs/brk). See [architecture](./ARCHITECTURE.md) for how they fit together.

## Architecture

```
blk*.dat ──▶ Reader ──┐
                      ├──▶ Indexer ──▶ Computer ──┐
         RPC Client ──┤                           ├──▶ Query ──▶ Server
                      └──▶ Mempool ───────────────┘
```

**Reader** parses Bitcoin Core's block files. **Indexer** builds lookup tables. **Computer** derives metrics. **Mempool** tracks unconfirmed transactions. **Query** provides unified data access. **Server** exposes the REST API.

[Detailed architecture](./ARCHITECTURE.md) · [All crates](https://docs.rs/brk)

## Links

- [Changelog](./CHANGELOG.md)
- [Support](./SUPPORT.md)
- [Contributing](https://github.com/bitcoinresearchkit/brk/issues)
- Community: [Discord](https://discord.gg/WACpShCB7M) · [Nostr](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6)
- Development supported by [OpenSats](https://opensats.org/)

## License

[MIT](./LICENSE.md)
