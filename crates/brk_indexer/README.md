# BRK Indexer

A [Bitcoin Core](https://bitcoincore.org/en/about/) node indexer which iterates over the chain (via `../brk_parser`) and creates a database of the vecs (`../brk_vec`) and key/value stores ([`fjall`](https://crates.io/crates/fjall)) that can be used in your Rust code.

The crate only stores the bare minimum to be self sufficient and not have to use an RPC client (except for scripts which are not stored). If you need more data, checkout `../computer` which uses the outputs from the indexer to compute a whole range of datasets.

Vecs are used sparingly instead of stores for multiple reasons:

- Only stores the relevant data since the key is an index
- Saved as uncompressed bytes and thus can be parsed manually (with any programming language) without relying on a server or library
- Easy to work with and predictable

## Usage

Storage wise, the expected overhead should be around 30% of the chain itself.

Peaks at 11-13 GB of RAM

## Outputs

Vecs: `src/storage/vecs/mod.rs`

Stores: `src/storage/stores/mod.rs`

## Examples

Rust: `src/main.rs`

Python: `../python/parse.py`
