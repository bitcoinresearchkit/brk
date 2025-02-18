# Indexer

A [Bitcoin Core](https://bitcoincore.org/en/about/) node indexer which iterates over the chain (via `../iterator`) and creates a database of the vecs (`../storable_vec`) and key/value stores ([`fjall`](https://crates.io/crates/fjall)) that can used in your Rust code.

The crate only stores the bare minimum to be self sufficient and not have to use an RPC client (except for scripts which are not stored). If you need more data, checkout `../computer` which uses the outputs from the indexer to compute a whole range of datasets.

The neat thing about using simple vecs to store data is that you can parse the outputed files with another programming language such as Python very simply, you'll find an example below.

## Usage

Rust: `src/main.rs`

Python: `../python/parse.py`

## Outputs

Vecs: `src/storage/storable_vecs/mod.rs`

Stores: `src/storage/fjalls/mod.rs`
