# brk_reader

Streams Bitcoin blocks from Bitcoin Core's raw `blk*.dat` files in
canonical chain order, skipping orphans.

## Requirements

A running Bitcoin Core node with RPC access. The reader needs:

- The `blocks/` directory (to read `blk*.dat` files)
- RPC connection (to resolve the canonical chain up front)

## Quick Start

```rust,ignore
let bitcoin_dir = Client::default_bitcoin_path();
let client = Client::new(
    Client::default_url(),
    Auth::CookieFile(bitcoin_dir.join(".cookie")),
)?;
let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

// Everything from genesis to the current tip
for block in reader.after(None)?.iter() {
    println!("{}: {}", block.height(), block.hash());
}

// Everything strictly after a known hash (typical sync / catchup pattern)
for block in reader.after(Some(last_known_hash))?.iter() {
    // ...
}

// A specific inclusive height range
for block in reader.range(Height::new(800_000), Height::new(850_000))?.iter() {
    // ...
}
```

`Reader` is thread-safe and cheap to clone (Arc-backed).

## What You Get

Each `ReadBlock` gives you access to:

| Field                  | Description                              |
| ---------------------- | ---------------------------------------- |
| `block.height()`       | Block height                             |
| `block.hash()`         | Block hash                               |
| `block.header`         | Block header (timestamp, nonce, ...)     |
| `block.txdata`         | All transactions                         |
| `block.coinbase_tag()` | Miner's coinbase tag                     |
| `block.metadata()`     | Position in the blk file                 |
| `block.tx_metadata()`  | Per-transaction blk file positions       |

## How It Works

Two-stage pipeline, one reader thread plus `N` parser threads
(default `N = 1`, configurable via `after_with` / `range_with`):

```text
canonical chain ──► Reader thread ──► Parser pool ──► Receiver<ReadBlock>
(pre-fetched         walks blk files,    N workers       in canonical order
 hashes via RPC)     peeks headers,      decode bodies
                     ships hits
```

1. **`CanonicalRange`** asks bitcoind once, up front, for the canonical
   block hash at every height in the target window — one batched
   JSON-RPC call, no per-block RPC chatter.
2. **Reader thread** walks blk files in order, scans each for block
   magic, and for every block found hashes its 80-byte header and
   looks the hash up in the canonical map. Orphans short-circuit
   before the block bytes are cloned.
3. **Parser pool** (scoped threads) fully decodes canonical bodies in
   parallel and serialises output through an in-order reorder buffer.
   The consumer always receives blocks in canonical-height order.

Orphans can never be mistaken for canonical blocks, and a missing
canonical block produces a hard error instead of a silent drop. See
`src/pipeline.rs` for the orchestration and `src/canonical.rs` for the
filter map.
