# Bitcoin Research Kit

[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/brk.svg)](https://crates.io/crates/brk)
[![docs.rs](https://img.shields.io/docsrs/brk)](https://docs.rs/brk)

Open-source on-chain analytics for Bitcoin. Combines functionality of [Glassnode](https://glassnode.com) (on-chain metrics), [mempool.space](https://mempool.space) (block explorer), and [electrs](https://github.com/romanz/electrs) (address index) into a single self-hostable package. [Bitview](https://bitview.space) is an analytics interface built entirely on BRK.

## Data

**Blockchain** — Blocks, transactions, addresses, UTXOs.

**Metrics** — Supply distributions, holder cohorts, network activity, fee markets, mining, and market indicators (realized cap, MVRV, SOPR, NVT).

**Indexes** — Date, height, halving epoch, address type, UTXO age.

**Mempool** — Fee estimation, projected blocks, unconfirmed transactions.

## Usage

### Website

Browse metrics and charts visually. Use it free at [Bitview](https://bitview.space), no signup required.

### API

Query thousands of metrics and blockchain data in JSON or CSV. Freely accessible at [Bitview](https://bitview.space/api).

[Documentation](https://bitview.space/api) · [JavaScript](https://www.npmjs.com/package/brk-client) · [Python](https://pypi.org/project/brk-client) · [Rust](https://crates.io/crates/brk_client)

### Self-host

Run your own website and API. Private, verifiable, self-sovereign. Runs alongside Bitcoin Core.

[Guide](./HOSTING.md)

### Library

Build custom applications in Rust. Use the full stack or individual components (parser, indexer, computer, server).

[Reference](https://docs.rs/brk) · [Architecture](./ARCHITECTURE.md)

## Links

- [Changelog](./CHANGELOG.md)
- [Support](./SUPPORT.md)
- [Contributing](https://github.com/bitcoinresearchkit/brk/issues)

[Discord](https://discord.gg/WACpShCB7M) · [Nostr](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6)

Development supported by [OpenSats](https://opensats.org/).

## License

[MIT](./LICENSE.md)
