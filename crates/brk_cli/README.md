# BRK Cli

## Setup

### Hardware

#### Recommended

- [Latest base model Mac mini](https://www.apple.com/mac-mini/)
- [Thunderbolt 4 SSD enclosure](https://satechi.net/products/usb4-nvme-ssd-pro-enclosure/Z2lkOi8vc2hvcGlmeS9Qcm9kdWN0VmFyaWFudC80MDE4ODQ3MDA2NzI4OA==?queryID=7961465089021ee203a60db7e62e90d2)
- [2 TB NVMe SSD](https://shop.sandisk.com/products/ssd/internal-ssd/wd-black-sn850x-nvme-ssd?sku=WDS200T2X0E-00BCA0)

#### Minimum

To be determined

### Software

- Unix based operating system (Mac OS or Linux)
  - Ubuntu users need to install `open-ssl` via `sudo apt install libssl-dev pkg-config`
- [Bitcoin](https://bitcoin.org/en/full-node)
  - Example: `bitcoind -datadir="$HOME/.bitcoin" -blocksonly`
- [Rust](https://www.rust-lang.org/tools/install)
  - Install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Update: `rustup update`

### Install

```bash
cargo install brk_cli
```

If it's your first time running `brk`, it will need several information such as:

- `--bitcoindir PATH`: path to bitcoin core data directory, `???/bitcoin`
- `--outputdir PATH`: path to various outputs, if you have enough space on your main disk `~/.brk` is fine

Everything will be saved at `~/.brk/config.toml`, which will allow you to simply run `brk run` next time

If you need more options please run `brk -h` to see what parameters are available.

Here's an example

```bash
brk run --bitcoindir=~/Developer/bitcoin --outputdir=~/.brk
```

Then the easiest to let others access your server is to use `cloudflared` which will also cache requests. For more information go to: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/

### Update

```bash
cargo install brk_cli
```

or

```bash
cargo install cargo-update
cargo install-update -a
```
