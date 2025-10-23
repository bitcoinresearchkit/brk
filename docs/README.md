# Bitcoin Research Kit (BRK)

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
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/docs/CHANGELOG.md">
    <img src="https://img.shields.io/badge/changelog-docs-blue" alt="Changelog" />
  </a>
</p>

> **The open-source alternative to expensive Bitcoin analytics platforms**
> Parse, index, analyze, and visualize Bitcoin blockchain data with unparalleled performance and zero restrictions.

---

## What is BRK?

The Bitcoin Research Kit is a **high-performance, open-source toolchain** that transforms raw Bitcoin blockchain data into actionable insights. Think of it as your complete Bitcoin data analytics stack—combining the power of Glassnode's metrics, mempool.space's real-time data, and electrs's indexing capabilities into one unified, freely accessible platform.

**Why BRK exists:** Existing Bitcoin analytics platforms are either prohibitively expensive (some costing thousands per month) or severely limited in functionality. Most are closed-source black boxes that contradict Bitcoin's principles of transparency and verifiability. BRK changes that.

## Key Features

- **Lightning Fast**: Built in Rust for maximum performance
- **![Datasets variant count](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fbitcoinresearchkit.org%2Fapi%2Fvecs%2Fvec-count&query=%24&style=flat&label=%20&color=white) Dataset Variants**: Comprehensive Bitcoin metrics out of the box
- **Completely Free**: No API limits, no paywalls, no accounts required
- **100% Open Source**: Fully auditable and verifiable
- **Multiple Interfaces**: Web UI, REST API, CLI, AI integration, and Rust crates
- **Self-Hostable**: Run your own instance with full control
- **AI-Ready**: Built-in LLM integration via Model Context Protocol

## Who Is This For?

| **Researchers**                              | **Developers**                           | **Miners**                                       | **Enthusiasts**                              |
| -------------------------------------------- | ---------------------------------------- | ------------------------------------------------ | -------------------------------------------- |
| Free access to comprehensive Bitcoin metrics | REST API and Rust crates for integration | Mining pool analytics and profitability tracking | Charts, visualizations, and network insights |
| Historical data analysis                     | High-performance indexing capabilities   | Difficulty and hashrate monitoring               | Educational blockchain exploration           |
| Academic research tools                      | Custom dataset creation                  | Fee market analysis                              | Portfolio and address tracking               |

## Quick Start

### 1. **Try it Online** (Fastest)

Visit **[bitview.space](https://bitview.space)** - No installation required, full feature access

### 2. **Use the API** (Developers)

```bash
# Get latest block height
curl https://bitcoinresearchkit.org/api/vecs?index=height&ids=height&from=-1

# Get Bitcoin price history
curl https://bitcoinresearchkit.org/api/vecs?index=dateindex&ids=price_usd&from=-30&count=30
```

### 3. **Self-Host** (Power Users)

```bash
# Install CLI
cargo install brk

# Run with your Bitcoin node
brk --bitcoindir /data/bitcoin --brkdir /data/brk
```

### 4. **AI Integration** (ChatGPT/Claude)

Connect your AI assistant to BRK's data using our [Model Context Protocol integration](https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_mcp/README.md).

## Use Cases

**Financial Analysis**

- Track on-chain metrics like transaction volume, active addresses, and HODL waves
- Analyze market cycles with realized cap, MVRV ratios, and spending patterns
- Monitor exchange flows and whale movements

**Mining Operations**

- Difficulty adjustment predictions and mining profitability analysis
- Pool distribution and hashrate monitoring
- Fee market dynamics and transaction prioritization

**Research & Development**

- Lightning Network adoption metrics
- UTXO set analysis and efficiency studies
- Protocol upgrade impact assessment

**Portfolio Management**

- Address and UTXO tracking
- Historical balance analysis
- Privacy and coin selection optimization

## Performance

BRK is designed for speed and efficiency:

- **Block Processing**: Parse entire blockchain in hours, not days
- **Query Performance**: Sub-millisecond response times for most metrics
- **Memory Efficiency**: Optimized data structures minimize RAM usage
- **Storage**: Compressed indexes reduce disk space requirements

## Contributing

Contributions from the Bitcoin community are welcome! Here's how to get involved:

1. **Report Issues**: Found a bug? [Open an issue](https://github.com/bitcoinresearchkit/brk/issues)
2. **Request Features**: Have an idea? We'd love to hear it
3. **Submit PRs**: Especially for new datasets and metrics
4. **Improve Docs**: Help make BRK more accessible
5. **Join Discussion**: [Discord](https://discord.gg/WACpShCB7M) | [Nostr](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6)

## Crates

| Crate                                                     | Purpose                                          |
| --------------------------------------------------------- | ------------------------------------------------ |
| [`brk`](https://crates.io/crates/brk)                     | Umbrella crate containing all BRK functionality  |
| [`brk_bundler`](https://crates.io/crates/brk_bundler)     | Web asset bundling (rolldown wrapper)            |
| [`brk_cli`](https://crates.io/crates/brk_cli)             | Command line interface for running BRK instances |
| [`brk_computer`](https://crates.io/crates/brk_computer)   | Bitcoin metrics and dataset computation          |
| [`brk_error`](https://crates.io/crates/brk_error)         | Error handling utilities                         |
| [`brk_fetcher`](https://crates.io/crates/brk_fetcher)     | Bitcoin price and market data fetcher            |
| [`brk_indexer`](https://crates.io/crates/brk_indexer)     | Blockchain data indexing engine                  |
| [`brk_logger`](https://crates.io/crates/brk_logger)       | Logging infrastructure                           |
| [`brk_mcp`](https://crates.io/crates/brk_mcp)             | Model Context Protocol bridge for AI integration |
| [`brk_query`](https://crates.io/crates/brk_query) | Data formatting and query interface              |
| [`brk_reader`](https://crates.io/crates/brk_reader)       | High-performance Bitcoin block parser            |
| [`brk_server`](https://crates.io/crates/brk_server)       | REST API server for data access                  |
| [`brk_store`](https://crates.io/crates/brk_store)         | Database storage abstraction (fjall wrapper)     |
| [`brk_types`](https://crates.io/crates/brk_types)     | Shared data structures                           |

## Documentation

- **[Changelog](https://github.com/bitcoinresearchkit/brk/blob/main/docs/CHANGELOG.md)** - Release history and version notes
- **[TODO](https://github.com/bitcoinresearchkit/brk/blob/main/docs/TODO.md)** - Planned features and improvements

## Professional Hosting

Need a managed BRK instance? We offer professional hosting services:

**What's Included:**

- Dual dedicated servers (1 GB/s) with redundant ISPs
- Cloudflare integration for global performance
- 99.99% uptime SLA
- Automatic updates and maintenance
- Direct support channel
- Custom Bitcoin Core/Knots versions
- Optional branded subdomains

**Pricing:** `0.01 BTC/month` or `0.1 BTC/year`

Contact: [hosting@bitcoinresearchkit.org](mailto:hosting@bitcoinresearchkit.org)

## Acknowledgments

This project is made possible by the generous support of:

- **[Open Sats](https://opensats.org/)**: Our primary grant provider, enabling full-time development since December 2024
- **Community Donors**: Supporters on [Nostr](https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44) and Geyser.fund who kept our public instance running before OpenSats

## Support the Project

Help us maintain and improve BRK:

**Bitcoin Address:**
[`bc1q09 8zsm89 m7kgyz e338vf ejhpdt 92ua9p 3peuve`](bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve)

**Other Ways to Support:**

- Star this repository
- Share BRK with your network
- Contribute code or documentation
- Join our community discussions

---

<p align="center">
  <strong>Built for the Bitcoin community</strong><br>
  <em>Open source • Free forever • No compromises</em>
</p>
