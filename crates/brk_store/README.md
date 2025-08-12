# brk_store

Thin wrapper around the Fjall embedded key-value store that provides typed, transactional storage for Bitcoin blockchain data with height-based versioning and batch operations. This crate adds BRK-specific functionality like height tracking, metadata management, and optimized configurations for Bitcoin data storage patterns.

## Features

- **Typed interface**: Generic over key and value types with automatic serialization
- **Height tracking**: Built-in blockchain height awareness for data versioning
- **Batch operations**: Efficient batch inserts and deletes with transaction support
- **Metadata management**: Automatic version and height metadata storage
- **Performance optimized**: Configured write buffers and memtable sizes for Bitcoin data
- **Bloom filters**: Configurable bloom filters for faster key lookups
- **Reset capability**: Clean store reset for reindexing operations

## Usage

```rust
use brk_store::{Store, open_keyspace};
use brk_structs::{Height, Version};
use std::path::Path;

fn main() -> brk_error::Result<()> {
    // Open the keyspace
    let keyspace = open_keyspace(Path::new("./data"))?;

    // Create a typed store
    let mut store: Store<String, u64> = Store::import(
        &keyspace,
        Path::new("./data"),
        "my_store",
        Version::ZERO,
        Some(true), // Enable bloom filters
    )?;

    // Insert data if needed at this height
    store.insert_if_needed(
        "key1".to_string(),
        42u64,
        Height::new(800_000)
    );

    // Commit changes
    store.commit(Height::new(800_000))?;

    // Persist to disk
    store.persist()?;

    // Query data
    if let Some(value) = store.get(&"key1".to_string())? {
        println!("Value: {}", value);
    }

    Ok(())
}
```

## Store Lifecycle

- **Import**: Create or open existing store with version checking
- **Insert**: Add key-value pairs with height-based conditional insertion
- **Commit**: Write batched changes to disk atomically
- **Persist**: Force sync all data to storage
- **Reset**: Clear all data for reindexing if needed

## AnyStore Trait

The `AnyStore` trait provides a type-erased interface for managing multiple stores:

```rust
use brk_store::AnyStore;

fn process_store(store: &mut dyn AnyStore) -> brk_error::Result<()> {
    if store.needs(Height::new(800_000)) {
        // Process this height
        store.commit(Height::new(800_000))?;
    }
    Ok(())
}
```
