use std::path::Path;

use brk_core::dot_brk_path;
use brk_indexer::Indexer;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let outputs_dir = dot_brk_path().join("outputs");
    // let outputs_dir = Path::new("../../_outputs");

    let compressed = false;

    let mut indexer = Indexer::new(outputs_dir.as_path(), compressed, true)?;
    indexer.import_stores()?;
    indexer.import_vecs()?;

    let height_to_timestamp = &indexer.vecs().height_to_timestamp;

    dbg!(height_to_timestamp.len());

    // height_to_timestamp.iter().for_each(|t| {
    //     dbg!(t);
    // });

    // let index = max_from.min(A::from(self.len()));
    let mut height_to_timestamp_iter = height_to_timestamp.iter();
    // height_to_timestamp.iter().for_each(|t| {
    //     dbg!(t);
    // });
    (0..2).for_each(|i| {
        dbg!(height_to_timestamp_iter.get_(i));
    });
    //     for_each(|t| {
    //     dbg!(t);
    // });
    // .try_for_each(|(height, timestamp)| -> Result<()> {
    //     let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
    //         dbg!((height, prev_h));
    //         let prev_timestamp = height_to_timestamp_iter
    //             .nth(prev_h.unwrap_to_usize())
    //             .context("To work")
    //             .inspect_err(|_| {
    //                 dbg!(prev_h);
    //             })
    //             .unwrap()
    //             .1
    //             .into_inner();
    //         timestamp
    //             .into_inner()
    //             .checked_sub(prev_timestamp)
    //             .unwrap_or(Timestamp::ZERO)
    //         // Ok(())
    //     });

    //     Ok(())
    //     // let (i, v) = t((a, b.into_inner(), self, &mut other_iter));
    //     // self.forced_push_at(i, v, exit)
    // })?;

    Ok(())
}
