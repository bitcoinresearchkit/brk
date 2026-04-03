//! Minimal reproduction: data written via start_ingestion is lost after close+reopen.
//!
//! This mimics what brk does:
//! 1. Open database with manual_journal_persist
//! 2. Create a keyspace (Kind::Recent config)
//! 3. Use start_ingestion to bulk-write data
//! 4. Call persist(SyncData)
//! 5. Drop the database
//! 6. Reopen
//! 7. Check if data survived

use brk_store::{Kind, Mode, Store};
use brk_types::{Height, TxIndex, TxidPrefix, Version};
use fjall::{Database, KeyspaceCreateOptions, PersistMode};

fn open_db(path: &std::path::Path) -> Database {
    Database::builder(path.join("fjall"))
        .cache_size(64 * 1024 * 1024)
        .open()
        .unwrap()
}

fn open_keyspace(db: &Database) -> fjall::Keyspace {
    db.keyspace("test_keyspace", || {
        KeyspaceCreateOptions::default()
            .manual_journal_persist(true)
            .expect_point_read_hits(true)
    })
    .unwrap()
}

#[test]
fn ingestion_survives_close_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();

    // Phase 1: write data via ingestion, then close
    {
        let db = open_db(path);
        let ks = open_keyspace(&db);

        let mut ingestion = ks.start_ingestion().unwrap();
        for i in 0u64..1000 {
            ingestion
                .write(i.to_be_bytes(), i.to_be_bytes())
                .unwrap();
        }
        ingestion.finish().unwrap();

        // Verify data is readable before close
        assert!(!ks.is_empty().unwrap(), "keyspace should have data before close");
        assert!(ks.get(0u64.to_be_bytes()).unwrap().is_some(), "key 0 should exist before close");

        db.persist(PersistMode::SyncData).unwrap();

        // db + ks dropped here
    }

    // Phase 2: reopen and check
    {
        let db = open_db(path);
        let ks = open_keyspace(&db);

        assert!(
            !ks.is_empty().unwrap(),
            "BUG: keyspace is empty after close+reopen — ingested data lost"
        );
        assert!(
            ks.get(0u64.to_be_bytes()).unwrap().is_some(),
            "BUG: key 0 missing after close+reopen"
        );
        assert!(
            ks.get(999u64.to_be_bytes()).unwrap().is_some(),
            "BUG: key 999 missing after close+reopen"
        );
    }
}

/// Same test but with a keyspace clone (mimics take_pending_ingest capturing keyspace.clone())
#[test]
fn ingestion_via_cloned_keyspace_survives_close_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();

    {
        let db = open_db(path);
        let ks = open_keyspace(&db);

        // Clone the keyspace (like take_pending_ingest does)
        let ks_clone = ks.clone();

        let mut ingestion = ks_clone.start_ingestion().unwrap();
        for i in 0u64..1000 {
            ingestion
                .write(i.to_be_bytes(), i.to_be_bytes())
                .unwrap();
        }
        ingestion.finish().unwrap();

        // Clone used for persist (like fjall_db.persist in bg task)
        let db_clone = db.clone();
        db_clone.persist(PersistMode::SyncData).unwrap();

        // Drop order mimics Indexer: ks_clone dropped first, then db_clone, then ks, then db
        drop(ks_clone);
        drop(db_clone);
        drop(ks);
        drop(db);
    }

    {
        let db = open_db(path);
        let ks = open_keyspace(&db);

        assert!(
            !ks.is_empty().unwrap(),
            "BUG: keyspace is empty after close+reopen — cloned ingestion data lost"
        );
        assert!(
            ks.get(500u64.to_be_bytes()).unwrap().is_some(),
            "BUG: key 500 missing after close+reopen (cloned keyspace path)"
        );
    }
}

/// Mimics brk at scale: 20+ keyspaces, parallel intermediate commits (like par_iter_any_mut),
/// hundreds of batches, large data, bg thread ingest, drop-db-before-keyspaces order.
#[test]
fn many_keyspaces_parallel_commits_bg_ingest() {
    use rayon::prelude::*;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();

    const NUM_KEYSPACES: usize = 25;
    const INTERMEDIATE_BATCHES: u64 = 500;
    const KEYS_PER_BATCH: u64 = 10_000;
    const BG_KEYS_PER_KS: u64 = 10_000;

    {
        let db = open_db(path);

        let keyspaces: Vec<fjall::Keyspace> = (0..NUM_KEYSPACES)
            .map(|i| {
                db.keyspace(&format!("ks_{i}"), || {
                    let mut opts = KeyspaceCreateOptions::default()
                        .manual_journal_persist(true);
                    // Mix configs like brk does (Kind::Recent vs Kind::Random vs Kind::Vec)
                    if i % 3 == 0 {
                        opts = opts.expect_point_read_hits(true);
                    }
                    opts
                })
                .unwrap()
            })
            .collect();

        // Intermediate commits — PARALLEL across keyspaces (like par_iter_any_mut)
        for batch in 0..INTERMEDIATE_BATCHES {
            keyspaces.par_iter().for_each(|ks| {
                let start = batch * KEYS_PER_BATCH;
                let end = start + KEYS_PER_BATCH;
                let mut ing = ks.start_ingestion().unwrap();
                for i in start..end {
                    ing.write(i.to_be_bytes(), i.to_be_bytes()).unwrap();
                }
                ing.finish().unwrap();
            });
            db.persist(PersistMode::SyncData).unwrap();
        }

        let total_intermediate = INTERMEDIATE_BATCHES * KEYS_PER_BATCH;
        eprintln!("Wrote {total_intermediate} keys/ks × {NUM_KEYSPACES} keyspaces in {INTERMEDIATE_BATCHES} parallel batches");

        // take_pending_ingest: clone each keyspace + db, run on bg thread SEQUENTIALLY
        let ks_clones: Vec<_> = keyspaces.iter().map(|ks| ks.clone()).collect();
        let db_clone = db.clone();

        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));

            // Sequential ingestion per keyspace (like `for task in tasks { task()?; }`)
            for ks_clone in &ks_clones {
                let start = total_intermediate;
                let end = start + BG_KEYS_PER_KS;
                let mut ing = ks_clone.start_ingestion().unwrap();
                for i in start..end {
                    ing.write(i.to_be_bytes(), i.to_be_bytes()).unwrap();
                }
                ing.finish().unwrap();
            }

            db_clone.persist(PersistMode::SyncData).unwrap();
        });

        // sync_bg_tasks
        handle.join().unwrap();

        // Stores drop order: db first, then keyspaces (struct field order)
        drop(db);
        drop(keyspaces);
    }

    // Reopen and verify
    {
        let db = open_db(path);
        let total_intermediate = INTERMEDIATE_BATCHES * KEYS_PER_BATCH;

        for i in 0..NUM_KEYSPACES {
            let ks = db
                .keyspace(&format!("ks_{i}"), || {
                    KeyspaceCreateOptions::default().manual_journal_persist(true)
                })
                .unwrap();

            assert!(
                !ks.is_empty().unwrap(),
                "BUG: ks_{i} is empty after reopen"
            );

            // Intermediate data
            assert!(
                ks.get(0u64.to_be_bytes()).unwrap().is_some(),
                "BUG: ks_{i} key 0 missing"
            );
            assert!(
                ks.get((total_intermediate - 1).to_be_bytes()).unwrap().is_some(),
                "BUG: ks_{i} key {} missing", total_intermediate - 1
            );

            // Bg task data
            let bg_mid = total_intermediate + BG_KEYS_PER_KS / 2;
            assert!(
                ks.get(bg_mid.to_be_bytes()).unwrap().is_some(),
                "BUG: ks_{i} key {bg_mid} (bg) missing"
            );

            // Spot checks across the full range
            for check in [1u64, 100, 1_000, 10_000, 100_000, 1_000_000, 4_999_999] {
                if check < total_intermediate + BG_KEYS_PER_KS {
                    assert!(
                        ks.get(check.to_be_bytes()).unwrap().is_some(),
                        "BUG: ks_{i} key {check} missing"
                    );
                }
            }
        }

        eprintln!("All {NUM_KEYSPACES} keyspaces verified after reopen");
    }
}

/// Uses the ACTUAL brk Store<TxidPrefix, TxIndex> type with commit + take_pending_ingest.
/// This exercises the exact code path that brk uses.
#[test]
fn actual_store_commit_then_take_pending_ingest() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();

    let stores_path = path.join("stores");
    std::fs::create_dir_all(&stores_path).unwrap();

    let version = Version::new(29); // MAJOR_FJALL_VERSION(3) + VERSION(26)

    {
        let db = brk_store::open_database(&stores_path).unwrap();

        let mut store: Store<TxidPrefix, TxIndex> = Store::import_cached(
            &db,
            &stores_path,
            "txid_prefix_to_tx_index",
            version,
            Mode::PushOnly,
            Kind::Recent,
            5,
        )
        .unwrap();

        // Simulate intermediate commits (like Stores::commit every 1000 blocks)
        for batch in 0u64..500 {
            for i in (batch * 1000)..((batch + 1) * 1000) {
                let prefix = TxidPrefix::from(byteview::ByteView::from(i.to_be_bytes()));
                let tx_index = TxIndex::from(i as usize);
                store.insert(prefix, tx_index);
            }
            // AnyStore::commit
            brk_store::AnyStore::commit(&mut store, Height::from(batch as u32))?;
            db.persist(PersistMode::SyncData).unwrap();
        }

        let total_intermediate = 500_000u64;

        // Verify before take_pending_ingest
        let prefix_0 = TxidPrefix::from(byteview::ByteView::from(0u64.to_be_bytes()));
        assert!(store.get(&prefix_0).unwrap().is_some(), "key 0 should exist before take");

        // Simulate take_pending_ingest: add more data, then take
        for i in total_intermediate..(total_intermediate + 5_000) {
            let prefix = TxidPrefix::from(byteview::ByteView::from(i.to_be_bytes()));
            let tx_index = TxIndex::from(i as usize);
            store.insert(prefix, tx_index);
        }

        let task = store
            .take_pending_ingest(Height::from(943425u32))
            .unwrap();

        // Simulate bg thread
        let db_clone = db.clone();
        let handle = std::thread::spawn(move || {
            if let Some(task) = task {
                task().unwrap();
            }
            db_clone.persist(PersistMode::SyncData).unwrap();
        });
        handle.join().unwrap();

        // Drop order: db first, then store (like Stores struct)
        drop(db);
        drop(store);
    }

    // Reopen and verify
    {
        let db = brk_store::open_database(&stores_path).unwrap();

        let store: Store<TxidPrefix, TxIndex> = Store::import_cached(
            &db,
            &stores_path,
            "txid_prefix_to_tx_index",
            version,
            Mode::PushOnly,
            Kind::Recent,
            5,
        )
        .unwrap();

        assert!(
            !store.is_empty().unwrap(),
            "BUG: store is empty after reopen"
        );

        // Check intermediate data
        let prefix_0 = TxidPrefix::from(byteview::ByteView::from(0u64.to_be_bytes()));
        assert!(
            store.get(&prefix_0).unwrap().is_some(),
            "BUG: key 0 (intermediate) missing after reopen"
        );

        let prefix_mid = TxidPrefix::from(byteview::ByteView::from(250_000u64.to_be_bytes()));
        assert!(
            store.get(&prefix_mid).unwrap().is_some(),
            "BUG: key 250000 (intermediate) missing after reopen"
        );

        // Check bg task data
        let prefix_bg = TxidPrefix::from(byteview::ByteView::from(502_000u64.to_be_bytes()));
        assert!(
            store.get(&prefix_bg).unwrap().is_some(),
            "BUG: key 502000 (bg task) missing after reopen"
        );
    }

    Ok(())
}
