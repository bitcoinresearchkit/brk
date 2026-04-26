//! Pipeline step 2: turn `Fetched` raws into a typed diff for the Applier.
//!
//! Pure CPU work, no locks. Three classes of new tx are handled:
//! - **live**: already in `known`, skipped (no update needed)
//! - **revivable**: in the graveyard, resurrected from the tombstone
//! - **fresh**: decoded from `new_raws`, prevouts resolved against
//!   `known` or `parent_raws`, RBF detected from the raw tx
//!
//! Removals come from cross-referencing inputs (see `removed.rs`).

mod added;
mod pulled;
mod removed;

pub use added::Addition;
pub use pulled::Pulled;
pub use removed::Removal;

use brk_types::TxidPrefix;
use rustc_hash::FxHashSet;

use crate::{
    steps::fetcher::Fetched,
    stores::{TxGraveyard, TxStore},
};

pub struct Preparer;

impl Preparer {
    pub fn prepare(fetched: Fetched, known: &TxStore, graveyard: &TxGraveyard) -> Pulled {
        let Fetched {
            entries_info,
            mut new_raws,
            parent_raws,
        } = fetched;

        let mut added: Vec<Addition> = Vec::new();
        let mut live: FxHashSet<TxidPrefix> =
            FxHashSet::with_capacity_and_hasher(entries_info.len(), Default::default());

        for info in &entries_info {
            live.insert(TxidPrefix::from(&info.txid));

            if known.contains(&info.txid) {
                continue;
            }
            if let Some(tomb) = graveyard.get(&info.txid) {
                added.push(added::revived(info, tomb));
                continue;
            }
            let Some(raw) = new_raws.remove(&info.txid) else {
                continue;
            };
            added.push(added::fresh(info, raw, &parent_raws, known));
        }

        let removed = removed::classify(&live, &added, known);

        Pulled { added, removed }
    }
}
