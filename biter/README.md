# Biter

Biter (Bitcoin Block Iterator) is a very fast and simple Rust library which reads raw block files (*blkXXXXX.dat*) from Bitcoin Core Node and creates an iterator over all the requested blocks in sequential order (0, 1, 2, ...).

The element returned by the iterator is a tuple which includes the:
- Height: `usize`
- Block: `Block` (from `bitcoin-rust`)
- Block's Hash: `BlockHash` (also from `bitcoin-rust`)

## Example

```rust
use bitcoincore_rpc::{Auth, Client};

fn main() {
    let i = std::time::Instant::now();

    // Path to the Bitcoin data directory
    let data_dir = "../../bitcoin";

    // Path to the export directory where a mini blk indexer will be exported
    let export_dir = "./target";

    // Inclusive starting height of the blocks received, `None` for 0
    let start = Some(850_000);

    // Inclusive ending height of the blocks received, `None` for the last one
    let end = None;

    // RPC client to filter out forks
    let url = "http://localhost:8332";
    let auth = Auth::UserPass("satoshi".to_string(), "nakamoto".to_string());
    let rpc = Client::new(url, auth).unwrap();

    // Create channel receiver then iterate over the blocks
    biter::new(data_dir, export_dir, start, end, rpc)
        .iter()
        .for_each(|(height, _block, hash)| {
            println!("{height}: {hash}");
        });

    dbg!(i.elapsed());
}

```

## Requirements

Even though it reads *blkXXXXX.dat* files, it **needs** `bitcoind` to run with the RPC server to filter out block forks.

Peak memory should be around 500MB.

## Comparaison

|  | [biter](https://crates.io/crates/biter) | [bitcoin-explorer](https://crates.io/crates/bitcoin-explorer) | [blocks_iterator](https://crates.io/crates/blocks_iterator) |
| --- | --- | --- | --- |
| Run **with** `bitcoind` | Yes ✅ | No ❌ | Yes ✅ |
| Run **without** `bitcoind` | No ❌ | Yes ✅ | Yes ✅ |
| `0..=855_000` | 16mn40s | 17mn 46s | > 2h |
| `800_000..=855_000` | 2mn 53s (16mn40s if first run) | 3mn 2s | > 2h |

*Benchmarked on a Macbook Pro M3 Pro*
