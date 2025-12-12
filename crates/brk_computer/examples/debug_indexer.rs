use std::{env, path::Path};

use brk_indexer::Indexer;
use brk_types::{Height, P2PKHAddressIndex, P2SHAddressIndex, TxOutIndex, TypeIndex};
use vecdb::GenericStoredVec;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let reader_outputtype = indexer.vecs.txout.txoutindex_to_outputtype.create_reader();
    let reader_typeindex = indexer.vecs.txout.txoutindex_to_typeindex.create_reader();
    let reader_txindex = indexer.vecs.txout.txoutindex_to_txindex.create_reader();
    let reader_txid = indexer.vecs.tx.txindex_to_txid.create_reader();
    let reader_height_to_first_txoutindex = indexer.vecs.txout.height_to_first_txoutindex.create_reader();
    let reader_p2pkh = indexer.vecs.address.p2pkhaddressindex_to_p2pkhbytes.create_reader();
    let reader_p2sh = indexer.vecs.address.p2shaddressindex_to_p2shbytes.create_reader();

    // Check what's stored at typeindex 254909199 in both P2PKH and P2SH vecs
    let typeindex = TypeIndex::from(254909199_usize);

    let p2pkh_bytes = indexer
        .vecs
        .address.p2pkhaddressindex_to_p2pkhbytes
        .read(P2PKHAddressIndex::from(typeindex), &reader_p2pkh);
    println!("P2PKH at typeindex 254909199: {:?}", p2pkh_bytes);

    let p2sh_bytes = indexer
        .vecs
        .address.p2shaddressindex_to_p2shbytes
        .read(P2SHAddressIndex::from(typeindex), &reader_p2sh);
    println!("P2SH at typeindex 254909199: {:?}", p2sh_bytes);

    // Check first P2SH index at height 476152
    let reader_first_p2sh = indexer.vecs.address.height_to_first_p2shaddressindex.create_reader();
    let reader_first_p2pkh = indexer.vecs.address.height_to_first_p2pkhaddressindex.create_reader();
    let first_p2sh_at_476152 = indexer.vecs.address.height_to_first_p2shaddressindex.read(Height::from(476152_usize), &reader_first_p2sh);
    let first_p2pkh_at_476152 = indexer.vecs.address.height_to_first_p2pkhaddressindex.read(Height::from(476152_usize), &reader_first_p2pkh);
    println!("First P2SH index at height 476152: {:?}", first_p2sh_at_476152);
    println!("First P2PKH index at height 476152: {:?}", first_p2pkh_at_476152);

    // Check the problematic txoutindexes found during debugging
    for txoutindex_usize in [653399433_usize, 653399443_usize] {
        let txoutindex = TxOutIndex::from(txoutindex_usize);
        let outputtype = indexer
            .vecs
            .txout.txoutindex_to_outputtype
            .read(txoutindex, &reader_outputtype)
            .unwrap();
        let typeindex = indexer
            .vecs
            .txout.txoutindex_to_typeindex
            .read(txoutindex, &reader_typeindex)
            .unwrap();
        let txindex = indexer
            .vecs
            .txout.txoutindex_to_txindex
            .read(txoutindex, &reader_txindex)
            .unwrap();
        let txid = indexer
            .vecs
            .tx.txindex_to_txid
            .read(txindex, &reader_txid)
            .unwrap();

        // Find height by binary search
        let mut height = Height::from(0_usize);
        for h in 0..900_000_usize {
            let first_txoutindex = indexer
                .vecs
                .txout.height_to_first_txoutindex
                .read(Height::from(h), &reader_height_to_first_txoutindex);
            if let Ok(first) = first_txoutindex {
                if usize::from(first) > txoutindex_usize {
                    break;
                }
                height = Height::from(h);
            }
        }

        println!(
            "txoutindex={}, outputtype={:?}, typeindex={}, txindex={}, txid={}, height={}",
            txoutindex_usize,
            outputtype,
            usize::from(typeindex),
            usize::from(txindex),
            txid,
            usize::from(height)
        );
    }

    Ok(())
}
