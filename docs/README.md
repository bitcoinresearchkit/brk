# Bitcoin Research Kit

[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/brk.svg)](https://crates.io/crates/brk)
[![docs.rs](https://img.shields.io/docsrs/brk)](https://docs.rs/brk)
[![Supported by OpenSats](https://img.shields.io/badge/supported%20by-opensats-ff7b00)](https://opensats.org/)
[![Discord](https://img.shields.io/discord/1350431684562124850?label=Discord&logo=discord&color=5865F2)](https://discord.gg/WACpShCB7M)
[![X](https://img.shields.io/badge/@_nym21_-000000?logo=x)](https://x.com/_nym21_)
[![Nostr](https://img.shields.io/badge/Nostr-purple?logo=nostr)](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6)

> "Shout out to Bitcoin Research Kit and researchbitcoin.net. Two data sources for On-Chain Data. Couldn't recommend them highly enough."
> — James Check (CheckOnChain), [What Bitcoin Did #1000](https://www.whatbitcoindid.com/episodes/wbd1000-checkmate)

Open-source, self-hostable on-chain analytics for Bitcoin. Block explorer, address index, and thousands of metrics, all from your own node. No external APIs required.

Similar to [Glassnode](https://glassnode.com) + [mempool.space](https://mempool.space) + [electrs](https://github.com/romanz/electrs) + [UTXO Oracle](https://utxo.live/oracle/) in a single package.

[Bitview](https://bitview.space) is a free hosted instance of BRK.

## Data

**Blockchain:** Blocks, transactions, addresses, UTXOs.

**Metrics:** Supply distributions, holder cohorts, network activity, fee markets, mining, and market indicators (realized cap, MVRV, SOPR, NVT).

**Indexes:** Date, height, halving epoch, address type, UTXO age.

**Mempool:** Fee estimation, projected blocks, unconfirmed transactions.

## Usage

### Website

Browse metrics and charts at [bitview.space](https://bitview.space), no signup required.

### API

```bash
curl https://bitview.space/api/blocks/count/height
```

Query metrics and blockchain data in JSON or CSV.

[Documentation](https://bitview.space/api) · [JavaScript](https://www.npmjs.com/package/brk-client) · [Python](https://pypi.org/project/brk-client) · [Rust](https://crates.io/crates/brk_client) · [LLM-friendly schema](https://bitview.space/api#tag/server/GET/api.json)

### Self-host

```bash
cargo install --locked brk_cli && brk
```

Run your own website and API. Private, verifiable, self-sovereign. Runs alongside Bitcoin Core.

[Guide](https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_cli/README.md) · [Professional hosting](./PROFESSIONAL_HOSTING.md)

### Library

```bash
cargo add brk
```

Build custom applications in Rust. Use the full stack or individual components (parser, indexer, computer, server).

[Reference](https://docs.rs/brk) · [Architecture](./ARCHITECTURE.md)

## Supporters

- [OpenSats](https://opensats.org/) (December 2024 - June 2026)

[Become a supporter](mailto:support@bitcoinresearchkit.org)

## Donations

<a href="https://x.com/_Checkmatey_"><img src="https://pbs.twimg.com/profile_images/1657255419172253698/ncG0Gt8e_400x400.jpg" width="60" alt="Checkmate" title="Checkmate" style="border-radius:50%" /></a>
<a href="https://x.com/JohanMBergman"><img src="https://pbs.twimg.com/profile_images/1958587470120988673/7rlY5csu_400x400.jpg" width="60" alt="Johan" title="Johan" style="border-radius:50%" /></a>
<a href="https://x.com/clearmined1"><img src="https://pbs.twimg.com/profile_images/1657777901830541313/6OAaR8XF_400x400.png" width="60" alt="ClearMined" title="ClearMined" style="border-radius:50%" /></a>

<img src="./qr.png" alt="Bitcoin donate QR code" width="180" />

[`bc1q09 8zsm89 m7kgyz e338vf ejhpdt 92ua9p 3peuve`](bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve)

## Links

- [Changelog](./CHANGELOG.md)
- [Contributing](https://github.com/bitcoinresearchkit/brk/issues)

## Community

- [Discord](https://discord.gg/WACpShCB7M)
- [X](https://x.com/_nym21_)
- [Nostr](https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6)

## License

[MIT](./LICENSE.md)
