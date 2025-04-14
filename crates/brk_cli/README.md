# BRK Cli

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://kibo.money">
    <img alt="kibo.money" src="https://img.shields.io/badge/showcase-kib%C5%8D.money-orange">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk_cli">
    <img src="https://img.shields.io/crates/v/brk_cli" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_cli">
    <img src="https://img.shields.io/docsrs/brk_cli" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_cli" alt="Size" />
  <a href="https://deps.rs/crate/brk_cli">
    <img src="https://deps.rs/crate/brk_cli/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/HaR3wpH3nr">
    <img src="https://img.shields.io/discord/1350431684562124850?label=discord" alt="Discord" />
  </a>
  <a href="https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6">
    <img src="https://img.shields.io/badge/nostr-purple?link=https%3A%2F%2Fprimal.net%2Fp%2Fnprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6" alt="Nostr" />
  </a>
  <a href="https://bsky.app/profile/bitcoinresearchkit.org">
    <img src="https://img.shields.io/badge/bluesky-blue?link=https%3A%2F%2Fbsky.app%2Fprofile%2Fbitcoinresearchkit.org" alt="Bluesky" />
  </a>
  <a href="https://x.com/0xbrk">
    <img src="https://img.shields.io/badge/x.com-black" alt="X" />
  </a>
</p>

A command line interface to interact with the full Bitcoin Research Kit. It's built on top of every other create and gives the possility to use BRK using the terminal instead of Rust.

It has 2 commandes for now (other than `help` and `version`) which are `run` and `query`. The former is used to run the processing (indexer + computer) and/or the server. The latter uses `brk_query` as its backend just like to server to be able to get datasets via the terminal instead of the API. Both commands are very costumizable by having all the parameters of their Rust counterparts ([`run`](https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_cli/src/run.rs#L91-L147), [`query`](https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_query/src/params.rs)).

## Requirements

### Hardware

#### Recommended

- [Latest base model Mac mini](https://www.apple.com/mac-mini/)
- [Thunderbolt 4 SSD enclosure](https://satechi.net/products/usb4-nvme-ssd-pro-enclosure/Z2lkOi8vc2hvcGlmeS9Qcm9kdWN0VmFyaWFudC80MDE4ODQ3MDA2NzI4OA==?queryID=7961465089021ee203a60db7e62e90d2)
- [2 TB NVMe SSD](https://shop.sandisk.com/products/ssd/internal-ssd/wd-black-sn850x-nvme-ssd?sku=WDS200T2X0E-00BCA0)

#### Minimum

To be determined

### Software

- [Bitcoin](https://bitcoin.org/en/full-node)
- [Rust](https://www.rust-lang.org/tools/install)
- Unix based operating system (Mac OS or Linux)
  - Ubuntu users need to install `open-ssl` via `sudo apt install libssl-dev pkg-config`

## Download

### Binaries

You can find a pre-built binary for your operating system on the releases page ([link](https://github.com/bitcoinresearchkit/brk/releases/latest)).

### Cargo

```bash
# Install
cargo install brk # or `cargo install brk_cli`, the result is the same

# Update
cargo install brk # or `cargo install-update -a` if you have `cargo-update` installed
```

### Source

```bash
git clone https://github.com/bitcoinresearchkit/brk.git
cd brk/crates/brk
cargo run -r
```

## Usage

Run `brk -h` to view each available command and their respective description.

`-h` works also for commands, which mean that `brk run -h` will explain all the parameters of `brk run` for example.

Every parameter set for `brk run` will be saved at `~/.brk/config.toml`, which will allow you to simply run `brk run` next time.

Then the easiest to let others access your server is to use `cloudflared` which will also cache requests. For more information go to: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/
