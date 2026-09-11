use bitview_cohort::{AddrTypeId, ByAddrType, WithAddrTypes};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{LazySpotValuePerBlock, StoredSeries, import_stored};
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Cents, Height, Sats, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, CacheBudget, Database, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, Rw,
    StorageMode, WritableVec,
};

use crate::AllChainSources;

#[derive(Traversable)]
pub struct AvgAmountVecs<M: StorageMode = Rw> {
    /// Mean value of an output unspent at the represented block: unspent supply
    /// divided by unspent output count.
    pub utxo: WithAddrTypes<LazySpotValuePerBlock>,
    /// Mean balance of a funded address: unspent supply divided by funded
    /// address count.
    pub addr: WithAddrTypes<LazySpotValuePerBlock>,
    #[traversable(hidden)]
    utxo_source: ByAddrType<StoredSeries<Height, Sats, M>>,
    #[traversable(hidden)]
    addr_source: ByAddrType<StoredSeries<Height, Sats, M>>,
}

impl AvgAmountVecs {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        spot_price: &ReadableBoxedVec<Height, Cents>,
        all_chain: &AllChainSources,
        utxo_count: &impl ReadableCloneableVec<Height, StoredU64>,
        funded_addr_count: &impl ReadableCloneableVec<Height, StoredU64>,
    ) -> Result<Self> {
        let avg_utxo = all_chain.with_supply(
            "avg_utxo_amount_sats_source",
            Version::ZERO,
            utxo_count,
            |_, count, supply| supply / count,
        );
        let avg_addr = all_chain.with_supply(
            "avg_addr_amount_sats_source",
            Version::ZERO,
            funded_addr_count,
            |_, count, supply| supply / count,
        );
        let utxo_source = ByAddrType::try_from_fn(|id| {
            import_stored(
                cache,
                db,
                &format!("{}_avg_utxo_amount_sats", id.name()),
                version + Version::ONE,
            )
        })?;
        let addr_source = ByAddrType::try_from_fn(|id| {
            import_stored(
                cache,
                db,
                &format!("{}_avg_addr_amount_sats", id.name()),
                version + Version::ONE,
            )
        })?;
        let utxo = WithAddrTypes {
            all: LazySpotValuePerBlock::from_sats_source(
                "avg_utxo_amount",
                version,
                &avg_utxo,
                mappings,
                spot_price,
            ),
            by_addr_type: AddrTypeId::series(|id, type_name| {
                LazySpotValuePerBlock::from_sats_source(
                    &format!("{type_name}_avg_utxo_amount"),
                    version,
                    id.select(&utxo_source),
                    mappings,
                    spot_price,
                )
            }),
        };
        let addr = WithAddrTypes {
            all: LazySpotValuePerBlock::from_sats_source(
                "avg_addr_amount",
                version,
                &avg_addr,
                mappings,
                spot_price,
            ),
            by_addr_type: AddrTypeId::series(|id, type_name| {
                LazySpotValuePerBlock::from_sats_source(
                    &format!("{type_name}_avg_addr_amount"),
                    version,
                    id.select(&addr_source),
                    mappings,
                    spot_price,
                )
            }),
        };

        Ok(Self {
            utxo,
            addr,
            utxo_source,
            addr_source,
        })
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.utxo_source
            .iter_mut()
            .chain(self.addr_source.iter_mut())
            .map(|(_, v)| v as &mut dyn AnyStoredVec)
            .collect::<Vec<_>>()
            .into_par_iter()
    }

    pub fn reset_height(&mut self) -> Result<()> {
        for (_, target) in self
            .utxo_source
            .iter_mut()
            .chain(self.addr_source.iter_mut())
        {
            target.reset()?;
        }
        Ok(())
    }

    pub fn compute(
        &mut self,
        supply_sats: &ByAddrType<&impl ReadableVec<Height, Sats>>,
        utxo_count: &ByAddrType<&impl ReadableVec<Height, StoredU64>>,
        funded_addr_count: &ByAddrType<&impl ReadableVec<Height, StoredU64>>,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        for &id in AddrTypeId::ALL {
            id.select_mut(&mut self.utxo_source).compute_transform2(
                max_from,
                *id.select(supply_sats),
                *id.select(utxo_count),
                |(height, supply, count, _)| (height, supply / count),
                exit,
            )?;
            id.select_mut(&mut self.addr_source).compute_transform2(
                max_from,
                *id.select(supply_sats),
                *id.select(funded_addr_count),
                |(height, supply, count, _)| (height, supply / count),
                exit,
            )?;
        }

        Ok(())
    }
}
