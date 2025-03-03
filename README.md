# Bitcoin Research Kit

The Bitcoin Research Kit is a suite of tools designed to extract, compute, serve and display data stored on a Bitcoin Core node.

In other words it's an alternative to [Glassnode](https://glassnode.com), [mempool.space](https://mempool.space/) and [electrs](https://github.com/romanz/electrs) all in one package with a particular focus on the self-hosting experience.

The toolkit can be used in various ways to accommodate as many needs as possible.

- **Website** \
  Anyone can go to [kibo.money](https://kibo.money) website which is good showcase if what the suite is capable of \
  Completely free including the API which is also auth-less \
  Powered by BRK using the `kibo.money` front-end and all features enabled
- **CLI** \
  Node runners can self-host their own instance using [`brk_cli`](https://crates.io/crates/brk_cli)
- **Crates** \
  Rust developers have a wide range crates available (which can be found below) for maximum flexibility

So whether you're an enthusiast, a researcher, a miner, an analyst, or just curious, there should be something for everyone !

## Crates

- [`brk`](https://crates.io/crates/brk): Wrapper around all other `brk-*` crates
- [`brk_cli`](https://crates.io/crates/brk_cli): A standalone command line interface to interact with the Bitcoin Research Kit
- [`brk_computer`](https://crates.io/crates/brk_computer): A Bitcoin dataset computer, built on top of brk_indexer
- [`brk_core`](https://crates.io/crates/brk_core): The Core (Structs and Errors) of the Bitcoin Research Kit
- [`brk_exit`](https://crates.io/crates/brk_exit): An exit blocker built on top of ctrlc
- [`brk_fetcher`](https://crates.io/crates/brk_fetcher): A Bitcoin price fetcher
- [`brk_indexer`](https://crates.io/crates/brk_indexer): A Bitcoin Core indexer built on top of brk_parser
- [`brk_logger`](https://crates.io/crates/brk_logger): A clean logger used in the Bitcoin Research Kit.
- [`brk_parser`](https://crates.io/crates/brk_parser): A very fast Bitcoin Core block parser and iterator built on top of bitcoin-rust
- [`brk_query`](https://crates.io/crates/brk_query): A library that finds requested datasets.
- [`brk_server`](https://crates.io/crates/brk_server): A server that serves Bitcoin data and swappable front-ends, built on top of `brk_indexer`, `brk_fetcher` and `brk_computer`
- [`brk_vec`](https://crates.io/crates/brk_vec): A very small, fast, efficient and simple storable Vec.

## Acknowledgments

Deepest gratitude to the [Open Sats](https://opensats.org/) public charity. Their grant — from December 2024 to the present — has been critical in sustaining this project.

Heartfelt thanks go out to every donor on [Nostr](https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44) and [Geyser.fund](https://geyser.fund/project/brk) whose support has ensured the availability of the [kibō.money](https://kibo.money) public instance.

## Hosting as a service

*Soon™*

If you'd like to have your own instance hosted for you please contact [tranche.dent-9o@icloud.com](mailto:tranche.dent-9o@icloud.com).

- Separate dedicated servers using different ISPs and Cloudflare for maximum availability 
- Optional `*.bitcoinresearchkit.org`, `*.kibo.money` and `*.satonomics.xyz` subdomains
- A very generous `2 * 1 GB/s` API limit
- Direct contact and feature requests

Pricing: `0.01 BTC / month` *or* `0.1 BTC / year`

## Donate

<img width="159" alt="image" src="https://github.com/user-attachments/assets/8bbb759f-4874-46cb-b093-b30cb30f5828">

[bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt](bitcoin:bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt)

<img width="159" alt="image" src="https://github.com/user-attachments/assets/745e39c7-be26-4f2a-90f2-54786e62ba35">

[lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4](lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4)

[Geyser Fund](https://geyser.fund/project/brk)

## Old README

## Endpoints

> If you running locally, you can replace `https://kibo.money` by `http://localhost:3110`

- [/](https://kibo.money/): Website
- [/api](https://kibo.money/api): A JSON with all available datasets, with their respective id and endpoint, better viewed in a Firefox based browser
- /api/TIMESCALE-to-ID: `TIMESCALE` can be `date` or `height`, and `ID` is the id with `_` replaced by `-`, let's take `date-to-close` (price at the end of each day) as an example
  - [/api/date-to-close](https://kibo.money/api/date-to-close): current year's values in a json format
  - [/api/date-to-close?chunk=2009](https://kibo.money/api/date-to-close?chunk=2009): values from the year 2009 in a json format
  - [/api/date-to-close?all=true](https://kibo.money/api/date-to-close?all=true): all values in a json format
  - You can also specify the extension to download a file, either `.json` or `.csv` to get the dataset in a CSV format; like so:
    - [/api/date-to-close.csv](https://kibo.money/api/date-to-close.csv)
    - [/api/date-to-close.csv?chunk=2009](https://kibo.money/api/date-to-close.csv?chunk=2009)
    - [/api/date-to-close.csv?all=true](https://kibo.money/api/date-to-close.csv?all=true)
