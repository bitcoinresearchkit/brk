# brk_monitor

A lightweight, thread-safe Rust library for maintaining a live, in-memory snapshot of the Bitcoin mempool.

## Key Features

- **Real-time synchronization**: Polls Bitcoin Core RPC every second to track mempool state
- **Thread-safe access**: Uses `RwLock` for concurrent reads with minimal contention
- **Efficient updates**: Only fetches new transactions, with configurable rate limiting (10,000 tx/cycle)
- **Zero-copy reads**: Exposes mempool via read guards for lock-free iteration
- **Optimized data structures**: Uses `FxHashMap` for fast lookups and minimal hashing overhead
- **Automatic cleanup**: Removes confirmed/dropped transactions on each update

## Design Principles

- **Minimal lock duration**: Lock held only during HashSet operations, never during I/O
- **Memory efficient**: Stores only missing txids during fetch phase
- **Simple API**: Just `new()`, `start()`, and `get_txs()`
- **Production-ready**: Error handling with logging, graceful degradation

## Use Cases

- Fee estimation and mempool analysis
- Transaction monitoring and alerts
- Block template prediction
- Network research and statistics

## Description

A clean, performant way to keep Bitcoin's mempool state available in your Rust application without repeatedly querying RPC. Perfect for applications that need frequent mempool access with low latency.
