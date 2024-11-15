
<a href="https://kibo.money" target="_blank">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/logo-full-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/logo-full-light.svg">
    <img alt="kibō" src="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/logo-full-light.svg" width="300" height="auto">
  </picture>
</a>

## Description

[**kibō**](https://kibo.money) (_hope_ in japanese) is primarily an open source Bitcoin Core data extractor and visualizer (similar to [Glassnode](https://glassnode.com)).

The project is split in 3 parts:

- First you have the extractor (parser), which parses the block data files from your Bitcoin Core node and computes a very wide range of datasets which are stored in compressed binary files
  > For the curious, it takes at the very least 24 hours to parse all the blocks and compute all datasets. After that it will wait for a new block and take between 1 and 3 minutes to be up to date
- Then there is the website on which you can view, among other things, all datasets in various charts
- Finally there is the server which serves the website and the genereted data via an [API](https://github.com/kibo-money/kibo/edit/main/README.md#endpoints)

Whether you're an enthusiast, a researcher, a miner, an analyst, a trader, a skeptic or just curious, there is something for everyone !

This project was created out of frustration by all the alternatives that were either very expensive and thus discriminatory and against bitcoin values or just very limited and none were open-source and verifiable. So while it's not the first tool trying to solve these problems, it's the first that is completely free, open-source and self-hostable. 

If you are a user of [mempool.space](https://mempool.space), you'll find this to be very complimentary, as it offers a macro view of the chain over time instead of a detailed one.

## Instances

| URL                                              | Type   | Version                                                                                                                                                                         | Status                                                                                                                                                         | Last Height                                                                                                                                                                      | Up Time Ratio                                                                                                                        |
| ------------------------------------------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| [kibo.money](https://kibo.money)                 | Main   | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fkibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03)         | ![Status](https://img.shields.io/uptimerobot/status/m797259009-043f6b92d4cc2deef7d13f50?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fkibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610)         | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797259009-043f6b92d4cc2deef7d13f50?style=for-the-badge&label=%20&color=%232f73f1) |
| [backup.kibo.money](https://backup.kibo.money)   | Backup | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fbackup.kibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03)  | ![Status](https://img.shields.io/uptimerobot/status/m797259013-bb29a8264fab8786fb80c5ed?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fbackup.kibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610)  | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797259013-bb29a8264fab8786fb80c5ed?style=for-the-badge&label=%20&color=%232f73f1) |
| [preview.kibo.money](https://preview.kibo.money) | Dev    | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fpreview.kibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03) | ![Status](https://img.shields.io/uptimerobot/status/m797869753-d40fc161bcb34624857a8082?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fpreview.kibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610) | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797869753-d40fc161bcb34624857a8082?style=for-the-badge&label=%20&color=%232f73f1) |

Please open an issue if you want to add another instance

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

## Roadmap

- **More Datasets/Charts**
- **Simulations**
- **Nostr integration**
- **API Documentation**
- **Descriptions**
- **Docker support**
- **Start9 support**

## Setup

### Requirements

- At least 16 GB of RAM
- 1 TB of free space (will use 70% of that without defragmentation and 40% after)
- A running instance of bitcoin-core with:
  - `-txindex=1`
  - `-blocksxor=0`
  - RPC credentials
  - Example: `bitcoind -datadir="$HOME/.bitcoin" -blocksonly -txindex=1 -blocksxor=0 -rpcuser="satoshi" -rpcpassword="nakamoto"`
- Git

### Manual

_Mac OS and Linux only, Windows is unsupported_

First we need to install Rust (https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already had Rust installed you could update it just in case

```bash
rustup update
```

> If you're on Ubuntu you'll probably also need to install `open-ssl` with
>
> ```bash
> sudo apt install libssl-dev pkg-config
> ```

Optionally, you can also install `cargo-watch` for the server to automatically restart it on file change, which will be triggered by new code and new datasets from the parser (https://github.com/watchexec/cargo-watch?tab=readme-ov-file#install)

```bash
cargo install cargo-watch --locked
```

Then you need to choose a path where all files related to **kibō** will live

```bash
cd ???
```

We can now clone the repository

```bash
git clone https://github.com/kibo-money/kibo.git
```

In a new terminal, go to the `parser`'s folder of the repository

```bash
cd ???/kibo/parser
```

Now we can finally start by running the parser, you need to use the `./run.sh` script instead of `cargo run -r` as we need to set various system variables for the program to run smoothly

For the first launch, the parser will need several information such as:

- `--datadir`: which is bitcoin data directory path, prefer `$HOME` to `~` as the latter might not work
- `--rpcuser`: the username of the RPC credentials to talk to the bitcoin server
- `--rpcpassword`: the password of the RPC credentials

Optionally you can also specify:

- `--rpcconnect`: if the bitcoin core server's IP is different than `localhost`
- `--rpcport`: if the port is different than `8332`

Everything will be saved in a `config.toml` file, which will allow you to simply run `./run.sh` next time

Here's an example

```bash
./run.sh --datadir=$HOME/Developer/bitcoin --rpcuser=satoshi --rpcpassword=nakamoto
```

In a **new** terminal, go to the `server`'s folder of the repository

```bash
cd ???/kibo/server
```

And start it also with the `run.sh` script instead of `cargo run -r`

```bash
./run.sh
```

Then the easiest to let others access your server is to use `cloudflared` which will also cache requests. For more information go to: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/

## Donate

<img width="159" alt="image" src="https://github.com/user-attachments/assets/8bbb759f-4874-46cb-b093-b30cb30f5828">

[bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt](bitcoin:bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt)

<img width="159" alt="image" src="https://github.com/user-attachments/assets/745e39c7-be26-4f2a-90f2-54786e62ba35">

[lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4](lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4)

[Geyser Fund](https://geyser.fund/project/kibo/)
