<p align="center">
  <a href="https://kibo.money" target="_blank">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/logo-full-dark.svg">
      <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/logo-full-light.svg">
      <img alt="kibō" src="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/logo-full-light.svg" width="300" height="auto" style="max-width: 100%;">
    </picture>
  </a>
</p>

<p align="center">
  <span>Bitcoin is our only <b><i>hope</i></b> for a better future.</span>
</p>

## Description

_TLDR_: **A better, FOSS, Bitcoin-only, self-hostable Glassnode**

**kibō** (_hope_ in japanese) is a suite of tools that aims to help you understand Bitcoin's various dynamics. To do that, there is a wide number of charts and datasets with a scale by date but also by height free for you to explore. Which allows you to verify an incredible number of things, from the number of UTXOs to the repartition of the supply between different groups over time, with many things in between and it's all made possible thanks to Bitcoin's transparency. Whether you're an enthusiast, a researcher, a miner, an analyst, a trader, a skeptic or just curious, there is something new to learn for everyone !

While it's not the first tool trying to solve this problem, it's the first that is completely free, open-source and self-hostable. Which is very important as, just like for Bitcoin itself, I strongly believe that everyone should have access to this kind of data.

If you are a user of [mempool.space](https://mempool.space), you'll find this to be very complimentary, as it's a global and macro view of the chain over time instead.

If we want the world to move towards and, in the end, to be on a Bitcoin standard, we must have tools like this at our disposal.

## Donations

This project was started as an answer to the outrageous pricing from Glassnode (and their third tier starting at $833.33/month !).

But it is a lot of work and has been worked on **full-time since November of 2023** and has also been operational since then without any ads.

**At the time of writing (2024-09-12), this project has made around 2,200,000 sats, which is around $1300 or $120/month. While I'm very grateful for all donations, it's sadly unsustainable.**

So if you find this project useful, [please send some sats](https://geyser.fund/project/kibo/), it would be really appreciated.

If you're a potential sponsor, feel free to contact me in Nostr !

Bitcoin address: [bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt](bitcoin:bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt)

Lightning address: lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4

[Geyser Fund](https://geyser.fund/project/kibo/)

## Warning

This project is still in an early stage. Until more people look at the code and check the various computations in it, the datasets might be, in the worst case, completely false.

## Instances

| URL                                              | Type   | Version                                                                                                                                                                         | Status                                                                                                                                                         | Last Height                                                                                                                                                                      | Up Time Ratio                                                                                                                        |
| ------------------------------------------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| [kibo.money](https://kibo.money)                 | Main   | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fkibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03)         | ![Status](https://img.shields.io/uptimerobot/status/m797259009-043f6b92d4cc2deef7d13f50?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fkibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610)         | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797259009-043f6b92d4cc2deef7d13f50?style=for-the-badge&label=%20&color=%232f73f1) |
| [backup.kibo.money](https://backup.kibo.money)   | Backup | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fbackup.kibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03)  | ![Status](https://img.shields.io/uptimerobot/status/m797259013-bb29a8264fab8786fb80c5ed?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fbackup.kibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610)  | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797259013-bb29a8264fab8786fb80c5ed?style=for-the-badge&label=%20&color=%232f73f1) |
| [preview.kibo.money](https://preview.kibo.money) | Dev    | ![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fpreview.kibo.money%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=%20&color=%23db9e03) | ![Status](https://img.shields.io/uptimerobot/status/m797869753-d40fc161bcb34624857a8082?style=for-the-badge&label=%20&up_color=%231cb454&down_color=%23e63636) | ![Height](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fpreview.kibo.money%2Fapi%2Flast-height.json&query=%24.value&style=for-the-badge&label=%20&color=%23f26610) | ![Ratio](https://img.shields.io/uptimerobot/ratio/m797869753-d40fc161bcb34624857a8082?style=for-the-badge&label=%20&color=%232f73f1) |

Please open an issue if you want to add another instance

## Structure

- `parser`: The backbone of the project, it does most of the work by parsing and then computing datasets from the timechain
  - Takes 2 to 4 days to parse the whole chain and create all datasets
  - If up to date wil take 1-3mn to compute the last 100 blocks
- `website`: A web app which displays the generated datasets in various charts
- `server`: A small server which will serve both the website and the computed datasets via an API

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
- **Dashboards**
- **Nostr integration**
- **API Documentation**
- **Descriptions**
- **Start9 support**

## Setup

### Requirements

- At least 16 GB of RAM
- 1 TB of free space (will use 60-80% of that)
- A running instance of bitcoin-core with:
  - `-txindex=1`
  - `-blocksxor=0`
  - RPC credentials
- Git

### Docker

Working on it

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

## Brand

### Name

kibō means _**hope**_ in japanese which is what Bitcoin ultimately is for many, hope for a better future.

Formerly Satonomics

### Logo

The dove (borrowed from [svgrepo](https://www.svgrepo.com/svg/351969/dove) for now) is known to represent hope in many cultures.

The orange background is a wink to Bitcoin and when in a circle, it also represents the sun, which means that while it's our hope for a better future, we still have to be careful with our collective goals and actions, to not end up like Icarus.
