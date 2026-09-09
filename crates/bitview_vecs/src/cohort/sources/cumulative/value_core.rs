use bitview_cohort::{Filter, UTXOCoreValues};
use bitview_transforms::{StoredU64ToCents, StoredU64ToSats};
use brk_error::Result;
use brk_types::{Cents, Height, Sats, StoredU64, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, LazyVec, Rw};

use super::CumulativeUTXOCoreSources;
use crate::SatsCents;

pub type CumulativeUTXOCoreValueSources<M = Rw> =
    SatsCents<CumulativeUTXOCoreSources<StoredU64, M>>;

impl CumulativeUTXOCoreValueSources {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            sats: CumulativeUTXOCoreSources::forced_import(
                cache,
                db,
                &format!("{name}_sats"),
                version,
            )?,
            cents: CumulativeUTXOCoreSources::forced_import(
                cache,
                db,
                &format!("{name}_cents"),
                version,
            )?,
        })
    }

    pub fn sources(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<(
        LazyVec<Height, Sats, Height, StoredU64>,
        LazyVec<Height, Cents, Height, StoredU64>,
    )> {
        Some((
            LazyVec::transformed::<StoredU64ToSats>(
                &format!("{name}_cumulative_sats"),
                version,
                self.sats.stored.get(filter)?.read_only_boxed_clone(),
            ),
            LazyVec::transformed::<StoredU64ToCents>(
                &format!("{name}_cumulative_cents"),
                version,
                self.cents.stored.get(filter)?.read_only_boxed_clone(),
            ),
        ))
    }

    pub fn push_block(
        &mut self,
        sats: impl Into<UTXOCoreValues<Sats>>,
        cents: impl Into<UTXOCoreValues<Cents>>,
    ) {
        self.sats
            .push_block(sats.into().map(|value| StoredU64::from(u64::from(*value))));
        self.cents
            .push_block(cents.into().map(|value| StoredU64::from(u64::from(*value))));
    }

    pub fn min_len(&self) -> usize {
        self.sats.min_len().min(self.cents.min_len())
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.sats.collect_vecs_mut();
        vecs.extend(self.cents.collect_vecs_mut());
        vecs
    }
}
