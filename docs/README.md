# Bitcoin Research Kit

Open-source on-chain analytics for Bitcoin.

[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/brk.svg)](https://crates.io/crates/brk)
[![docs.rs](https://img.shields.io/docsrs/brk)](https://docs.rs/brk)

Combines functionality of [Glassnode](https://glassnode.com) (on-chain metrics), [mempool.space](https://mempool.space) (block explorer), and [electrs](https://github.com/romanz/electrs) (address index) into a single self-hostable package. See [Bitview](https://bitview.space) for a live example.

## Data

**Blockchain** — Blocks, transactions, addresses, UTXOs.

**Metrics** — Supply distributions, holder cohorts, network activity, fee markets, mining, and market indicators (realized cap, MVRV, SOPR, NVT).

**Indexes** — Query by date, height, halving epoch, address type, UTXO age.

**Mempool** — Fee estimation, projected blocks, unconfirmed transactions.

## Usage

**API** — REST with JSON/CSV. [Documentation](https://bitcoinresearchkit.org/api). Clients: [JavaScript](https://www.npmjs.com/package/brk-client), [Python](https://pypi.org/project/brk-client), [Rust](https://crates.io/crates/brk_client).

**Self-host** — Requires Bitcoin Core. [Guide](./HOSTING.md). [Docker](https://github.com/bitcoinresearchkit/brk/tree/main/docker).

**Library** — [docs.rs/brk](https://docs.rs/brk). [Architecture](./ARCHITECTURE.md).

**MCP** — Model Context Protocol server for LLMs.

## Links

- [Changelog](./CHANGELOG.md)
- [Support](./SUPPORT.md)
- [Contributing](https://github.com/bitcoinresearchkit/brk/issues)

[Discord](https://discord.gg/WACpShCB7M) · [Nostr](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6)

Development supported by [OpenSats](https://opensats.org/).

## License

[MIT](./LICENSE.md)
