# brk_reader

Streams Bitcoin blocks from Bitcoin Core's raw `blk*.dat` files in chain order.

## Requirements

A running Bitcoin Core node with RPC access. The reader needs:
- The `blocks/` directory (for `blk*.dat` files)
- RPC connection (to resolve block heights and filter orphan blocks)

## Quick Start

```rust,ignore
let bitcoin_dir = Client::default_bitcoin_path();
let client = Client::new(
    Client::default_url(),
    Auth::CookieFile(bitcoin_dir.join(".cookie")),
)?;
let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

// Stream the entire chain
for block in reader.read(None, None) {
    println!("{}: {}", block.height(), block.hash());
}

// Or a specific range (inclusive)
for block in reader.read(Some(Height::new(800_000)), Some(Height::new(850_000))) {
    // ...
}
```

## What You Get

Each `ReadBlock` gives you access to:

| Field | Description |
|-------|-------------|
| `block.height()` | Block height |
| `block.hash()` | Block hash |
| `block.header` | Block header (timestamp, nonce, difficulty, ...) |
| `block.txdata` | All transactions |
| `block.coinbase_tag()` | Miner's coinbase tag |
| `block.metadata()` | Position in the blk file |
| `block.tx_metadata()` | Per-transaction blk file positions |

`Reader` is thread-safe and cheap to clone (Arc-backed).

## How It Works

Three-thread pipeline connected by bounded channels:

```text
blk*.dat ──► File Reader ──► Parser Pool ──► Orderer ──► Receiver<ReadBlock>
              1 thread        up to 4         1 thread
```

1. **File reader** binary-searches to the starting blk file, scans for magic bytes, segments raw blocks
2. **Parser pool** XOR-decodes and deserializes blocks in parallel, skips out-of-range blocks via header timestamp, filters orphans via RPC
3. **Orderer** buffers out-of-order arrivals, validates `prev_blockhash` continuity, emits blocks sequentially
