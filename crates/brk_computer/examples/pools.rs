use std::{collections::BTreeMap, path::Path, thread};

use brk_computer::Computer;
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_structs::{AddressBytes, OutputIndex, OutputType, pools};
use vecdb::{AnyIterableVec, Exit, VecIterator};

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || -> Result<()> {
            let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");

            let indexer = Indexer::forced_import(&outputs_dir)?;

            let fetcher = Fetcher::import(true, None)?;

            let computer = Computer::forced_import(&outputs_dir, &indexer, Some(fetcher))?;

            let pools = pools();

            let mut res: BTreeMap<&'static str, usize> = BTreeMap::default();

            let vecs = indexer.vecs;
            let stores = indexer.stores;

            let mut height_to_first_txindex_iter = vecs.height_to_first_txindex.iter();
            let mut txindex_to_first_outputindex_iter = vecs.txindex_to_first_outputindex.iter();
            let mut txindex_to_output_count_iter = computer.indexes.txindex_to_output_count.iter();
            let mut outputindex_to_outputtype_iter = vecs.outputindex_to_outputtype.iter();
            let mut outputindex_to_typeindex_iter = vecs.outputindex_to_typeindex.iter();
            let mut p2pk65addressindex_to_p2pk65bytes_iter =
                vecs.p2pk65addressindex_to_p2pk65bytes.iter();
            let mut p2pk33addressindex_to_p2pk33bytes_iter =
                vecs.p2pk33addressindex_to_p2pk33bytes.iter();
            let mut p2pkhaddressindex_to_p2pkhbytes_iter =
                vecs.p2pkhaddressindex_to_p2pkhbytes.iter();
            let mut p2shaddressindex_to_p2shbytes_iter = vecs.p2shaddressindex_to_p2shbytes.iter();
            let mut p2wpkhaddressindex_to_p2wpkhbytes_iter =
                vecs.p2wpkhaddressindex_to_p2wpkhbytes.iter();
            let mut p2wshaddressindex_to_p2wshbytes_iter =
                vecs.p2wshaddressindex_to_p2wshbytes.iter();
            let mut p2traddressindex_to_p2trbytes_iter = vecs.p2traddressindex_to_p2trbytes.iter();
            let mut p2aaddressindex_to_p2abytes_iter = vecs.p2aaddressindex_to_p2abytes.iter();

            let unknown = pools.get_unknown();

            stores
                .height_to_coinbase_tag
                .iter()
                .for_each(|(height, coinbase_tag)| {
                    let txindex = height_to_first_txindex_iter.unwrap_get_inner(height);
                    let outputindex = txindex_to_first_outputindex_iter.unwrap_get_inner(txindex);
                    let outputcount = txindex_to_output_count_iter.unwrap_get_inner(txindex);

                    let pool = (*outputindex..(*outputindex + *outputcount))
                        .map(OutputIndex::from)
                        .find_map(|outputindex| {
                            let outputtype =
                                outputindex_to_outputtype_iter.unwrap_get_inner(outputindex);
                            let typeindex =
                                outputindex_to_typeindex_iter.unwrap_get_inner(outputindex);

                            let address = match outputtype {
                                OutputType::P2PK65 => Some(AddressBytes::from(
                                    p2pk65addressindex_to_p2pk65bytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2PK33 => Some(AddressBytes::from(
                                    p2pk33addressindex_to_p2pk33bytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2PKH => Some(AddressBytes::from(
                                    p2pkhaddressindex_to_p2pkhbytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2SH => Some(AddressBytes::from(
                                    p2shaddressindex_to_p2shbytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2WPKH => Some(AddressBytes::from(
                                    p2wpkhaddressindex_to_p2wpkhbytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2WSH => Some(AddressBytes::from(
                                    p2wshaddressindex_to_p2wshbytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2TR => Some(AddressBytes::from(
                                    p2traddressindex_to_p2trbytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                OutputType::P2A => Some(AddressBytes::from(
                                    p2aaddressindex_to_p2abytes_iter
                                        .unwrap_get_inner(typeindex.into()),
                                )),
                                _ => None,
                            };

                            address
                                .and_then(|address| pools.find_from_address(&address.to_string()))
                        })
                        .or_else(|| pools.find_from_coinbase_tag(&coinbase_tag))
                        .unwrap_or(unknown);

                    *res.entry(pool.name).or_default() += 1;
                });

            let mut v = res.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
            v.sort_unstable();
            println!("{:#?}", v);
            println!("{:#?}", v.len());

            Ok(())
        })?
        .join()
        .unwrap()
}
