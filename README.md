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

**kibō** (*hope* in japanese, formerly Satonomics) is a suite of tools that aims to help you understand Bitcoin's various dynamics. To do that, there is a wide number of charts and datasets with a scale by date but also by height free for you to explore. Which allows you to verify an incredible number of things, from the number of UTXOs to the repartition of the supply between different groups over time, with many things in between and it's all made possible thanks to Bitcoin's transparency. Whether you're an enthusiast, a researcher, a miner, an analyst, a trader, a skeptic or just curious, there is something new to learn for everyone !

While it's not the first tool trying to solve this problem, it's the first that is completely free, open-source and self-hostable. Which is very important as, just like for Bitcoin itself, I strongly believe that everyone should have access to this kind of data.

If you are a user of [mempool.space](https://mempool.space), you'll find this to be very complimentary, as it's a global and macro view of the chain over time instead.

If we want the world to move towards and, in the end, to be on a Bitcoin standard, we must have tools like this at our disposal.

## Donations

This project was started as an answer to the outrageous pricing from Glassnode (and their third tier starting at $833.33/month !).

But it is a lot of work and has been worked on **full-time since November of 2023** and has also been operational since then without any ads.

**At the time of writing (2024-09-12), this project has made around 2,200,000 sats, which is around $1300 or $120/month. While I'm very grateful for all donations, it's sadly unsustainable.**

So if you find this project useful, [please send some sats](https://geyser.fund/project/kibo/), it would be really appreciated.

If you're a potential sponsor, feel free to contact me in Nostr !

[Geyser Fund](https://geyser.fund/project/kibo/)

## Warning

This project is still in an early stage. Until more people look at the code and check the various computations in it, the datasets might be, in the worst case, completely false.

## Instances

- [kibo.money](https://kibo.money)
- [backup.kibo.money](https://backup.kibo.money)

## Structure

- `parser`: The backbone of the project, it does most of the work by parsing and then computing datasets from the timechain.
- `website`: A web app which displays the generated datasets in various charts and dashboards.
- `server`: A small server which will serve both their resources

## Setup

### Requirements

- At least 16 GB of RAM
- 1 TB of free space (will use 60-80% of that)
- A running instance of bitcoin-core with txindex=1 and rpc credentials
- Git

### Docker

Coming soon

### Manual

First we need to install Rust (https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already had Rust installed you could update it just in case

```bash
rustup update
```

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
- `--datadir`: which is bitcoin data directory path
- `--rpcuser`: the username of the RPC credentials to talk to the bitcoin server
- `--rpcpassword`: the password of the RPC credentials

Everything will be saved in a `config.toml` file, which will allow you to simply run `./run.sh` next time

Here's an example

```bash
./run.sh --datadir=$HOME/Developer/bitcoin --rpcuser=satoshi --rpcpassword=nakamoto
```

In a new terminal, go to the `server`'s folder of the repository

```bash
cd ???/kibo/server
```

And start it also with the `run.sh` script instead of `cargo run -r`

```bash
./run.sh
```

Then the easiest to let others access your server is to use `cloudflared` which will also cache requests. For more information go to: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/

## Roadmap

- **More Datasets/Charts**: If a dataset can be computed, it should exist and have its related charts
- **Simulations**: Such as DCA simulation over a period of time with various strategies
- **Dashboards**: For a quick and real-time view of the latest data of all the datasets
- **Nostr integration**: First to save preferences, later to add some social functionnality
- **Descriptions**: Add text to describe all charts and what they mean
- **Start9 support**: By making the whole suite much easier to self-host, it's quite rough right now
- **API Documentation**: Highly needed to explain what's what
- **Datasets by block timestamp**: In addition to having datasets by date and height

## Philosophy

- **Best**: Replace Glassnode as the go to
- **Diverse**: Have as many charts/datasets as possible and something for everyone
- **Free**: Is and always will be completely free
- **Open**: With a very permissive license
- **Trustless**: You can verify and see exactly how each dataset is computed
- **Independent**: Only one, easily swappable, dependency (Price API)
- **Educational**: By providing many datasets that can be used to describe how Bitcoin works and why
- **Timeless**: Be relevant and usable 10 years from now by being independent and not do address tagging
- **Sovereign**: Be self-hostable on accessible hardware
- **Versatile**: You can view the data in charts, you can download the data, you can fetch the data via an API
- **Accessible**: Free Website and API with all the datasets for everyone

## Brand

### Name

kibō means _**hope**_ in japanese which is what Bitcoin ultimately is for many, hope for a better future.

### Logo

The dove (borrowed from [svgrepo](https://www.svgrepo.com/svg/351969/dove) for now) is known to represent hope in many cultures.

The orange background is a wink to Bitcoin and when in a circle, it also represents the sun, which means that while it's our hope for a better future, we still have to be careful with our collective goals and actions, to not end up like Icarus.

## Infrastructure

Here's the current infrastructure of the main instance and its backup.

It uses 2 servers, a full and a light one without the parser running but with still datasets syncronized via Syncthing.

Cloudflare is used for their tunnel + CDN services.

Though it's recommended to change to default **Browser Cache TTL** configuration from `4 Hours` to `Respect Existing Headers` (in `Websites / YOUR_DOMAIN / Caching / Configuration / Browser Cache TTL`) and activate `Always use https`.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/infrastructure-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/infrastructure-light.svg">
    <img alt="kibō" src="https://raw.githubusercontent.com/kibo-money/kibo/main/assets/infrastructure-light.svg" width="768" height="auto" style="max-width: 100%;">
  </picture>
</p>

## Iterations

A list of all the previous versions and ideas:

- https://github.com/drgarlic/satonomics
- https://github.com/drgarlic/satonomics-parser
- https://github.com/drgarlic/satonomics-explorer
- https://github.com/drgarlic/satonomics-server
- https://github.com/drgarlic/satonomics-app
- https://github.com/drgarlic/bitalisys
- https://github.com/drgarlic/bitesque-app
- https://github.com/drgarlic/bitesque-back
- https://github.com/drgarlic/bitesque-front
- https://github.com/drgarlic/bitesque-assets
- https://github.com/drgarlic/syf
