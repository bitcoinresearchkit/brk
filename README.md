# Bitcoin Research Kit

## Description

The Bitcoin Research Kit is a suite of tools designed to extract, compute and display data stored on a Bitcoin Core node.

- [`brk`](https://crates.io/crates/brk): Wrapper around all other `brk-*` crates
- [`brk_cli`](https://crates.io/crates/brk_cli): A command line interface to interact with the Bitcoin Research Kit
- [`brk_computer`](https://crates.io/crates/brk_computer): A Bitcoin dataset computer, built on top of brk_indexer
- [`brk_core`](https://crates.io/crates/brk_core): The Core (Structs and Errors) of the Bitcoin Research Kit
- [`brk_exit`](https://crates.io/crates/brk_exit): An exit blocker built on top of ctrlc
- [`brk_fetcher`](https://crates.io/crates/brk_fetcher): A Bitcoin price fetcher
- [`brk_indexer`](https://crates.io/crates/brk_indexer): A Bitcoin Core indexer built on top of brk_parser
- [`brk_logger`](https://crates.io/crates/brk_logger): A clean logger used in the Bitcoin Research Kit.
- [`brk_parser`](https://crates.io/crates/brk_parser): A very fast Bitcoin Core block parser and iterator built on top of bitcoin-rust
- [`brk_query`](https://crates.io/crates/brk_query): A library that finds requested datasets.
- [`brk_server`](https://crates.io/crates/brk_server): A server that serves Bitcoin data and swappable front-ends, built on top of brk_indexer, brk_fetcher and brk_computer
- [`brk_vec`](https://crates.io/crates/brk_vec): A very small, fast, efficient and simple storable Vec.

## Servers

| URL                                              | Front-end   | Version                                                                                                                                                                         | Status                                                                                                                                                         | Last Height                                                                                                                                                                      | Up Time Ratio                                                                                                                        |
| ------------------------------------------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| [kibo.money](https://kibo.money)                 | kibo.money   | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fkibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03)         | ![Status](https://img.shields.io/uptimerobot/status/m797259009-043f6b92d4cc2deef7d13f50?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fkibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610)         | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797259009-043f6b92d4cc2deef7d13f50?style=for-the-badge&label=%20&color=%232f73f1) |

Feel free to open an issue if you want to add another instance

## Setup

[Guide](https://crates.io/crates/brk_cli)

## Donate

<img width="159" alt="image" src="https://github.com/user-attachments/assets/8bbb759f-4874-46cb-b093-b30cb30f5828">

[bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt](bitcoin:bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt)

<img width="159" alt="image" src="https://github.com/user-attachments/assets/745e39c7-be26-4f2a-90f2-54786e62ba35">

[lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4](lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4)

[Geyser Fund](https://geyser.fund/project/kibo/)

## Old README

[**kibō**](https://kibo.money) (_hope_ in japanese) is primarily an open source Bitcoin Core data extractor and visualizer (similar to [Glassnode](https://glassnode.com)) which goal is to empower anybody with data about Bitcoin for free.

The project is split in 3 parts:

- First you have the extractor (parser), which parses the block data files from your Bitcoin Core node and computes a very wide range of datasets which are stored in compressed binary files
  > For the curious, it takes at the very least 24 hours to parse all the blocks and compute all datasets. After that it will wait for a new block and take between 1 and 3 minutes to be up to date
- Then there is the website on which you can view, among other things, all datasets in various charts
- Finally there is the server which serves the website and the generated data via an [API](https://github.com/kibo-money/kibo/tree/main#endpoints)

Whether you're an enthusiast, a researcher, a miner, an analyst, a trader, a skeptic or just curious, there is something for everyone !

This project was created out of frustration by all the alternatives that were either very expensive and thus discriminatory and against bitcoin values or just very limited and none were open-source and verifiable. So while it's not the first tool trying to solve these problems, it's the first that is completely free, open-source and self-hostable.

If you are a user of [mempool.space](https://mempool.space), you'll find this to be very complimentary, as it offers a macro view of the chain over time instead of a detailed one.

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
