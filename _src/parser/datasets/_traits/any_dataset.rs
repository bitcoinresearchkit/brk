use itertools::Itertools;
use rayon::prelude::*;
use struct_iterable::Iterable;

use crate::{
    parser::datasets::{
        cohort_metadata::AddressCohortMetadataDataset, ComputeData, DateRecapDataset, RatioDataset,
        SubDataset,
    },
    structs::{
        AnyBiMap, AnyDateMap, AnyHeightMap, AnyMap, BiMap, Date, DateMap, Height, HeightMap,
        MapKind, Timestamp, OHLC,
    },
};

use super::{AnyDatasetGroup, MinInitialStates};

pub trait AnyDataset: Iterable {
    fn get_min_initial_states(&self) -> &MinInitialStates;

    fn needs_insert(&self, height: Height, date: Date) -> bool {
        self.needs_insert_height(height) || self.needs_insert_date(date)
    }

    #[inline(always)]
    fn needs_insert_height(&self, height: Height) -> bool {
        !self.to_all_inserted_height_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .inserted
                .first_unsafe_height
                .unwrap_or(Height::ZERO)
                <= height
    }

    #[inline(always)]
    fn needs_insert_date(&self, date: Date) -> bool {
        !self.to_all_inserted_date_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .inserted
                .first_unsafe_date
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn to_kind_bi_map_vec(&self, kind: MapKind) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        let mut v = vec![];

        self.iter().for_each(|(_, any)| {
            if let Some(map) = any.downcast_ref::<BiMap<u8>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<u16>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<u32>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<u64>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<usize>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<f32>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<f64>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<OHLC>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<Date>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<Height>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<BiMap<Timestamp>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyBiMap + Send + Sync))
                }
            } else if let Some(dataset) = any.downcast_ref::<RatioDataset>() {
                match kind {
                    MapKind::Inserted => dataset.to_inserted_bi_map_vec(),
                    MapKind::Computed => dataset.to_computed_bi_map_vec(),
                }
                .into_iter()
                .for_each(|map| {
                    v.push(map);
                });
            } else if let Some(dataset) = any.downcast_ref::<AddressCohortMetadataDataset>() {
                match kind {
                    MapKind::Inserted => dataset.to_inserted_bi_map_vec(),
                    MapKind::Computed => dataset.to_computed_bi_map_vec(),
                }
                .into_iter()
                .for_each(|map| {
                    v.push(map);
                });
            } else if let Some(dataset) = any.downcast_ref::<SubDataset>() {
                dataset.as_vec().into_iter().for_each(|dataset| {
                    v.append(&mut dataset.to_kind_bi_map_vec(kind));
                });
            }
        });

        v
    }

    fn to_kind_mut_bi_map_vec(&mut self, kind: MapKind) -> Vec<&mut dyn AnyBiMap> {
        let mut v = vec![];

        self.iter_mut().for_each(|(_, any)| match any {
            any if any.is::<BiMap<u8>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<u8>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<u16>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<u16>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<u32>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<u32>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<u64>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<u64>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<usize>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<usize>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<f32>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<f32>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<f64>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<f64>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<OHLC>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<OHLC>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<Date>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<Date>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<Height>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<Height>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<BiMap<Timestamp>>() => {
                if let Some(map) = any.downcast_mut::<BiMap<Timestamp>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyBiMap);
                    }
                }
            }
            any if any.is::<RatioDataset>() => {
                if let Some(dataset) = any.downcast_mut::<RatioDataset>() {
                    match kind {
                        MapKind::Inserted => dataset.to_inserted_mut_bi_map_vec(),
                        MapKind::Computed => dataset.to_computed_mut_bi_map_vec(),
                    }
                    .into_iter()
                    .for_each(|map| {
                        v.push(map);
                    });
                }
            }
            any if any.is::<AddressCohortMetadataDataset>() => {
                if let Some(dataset) = any.downcast_mut::<AddressCohortMetadataDataset>() {
                    match kind {
                        MapKind::Inserted => dataset.to_inserted_mut_bi_map_vec(),
                        MapKind::Computed => dataset.to_computed_mut_bi_map_vec(),
                    }
                    .into_iter()
                    .for_each(|map| {
                        v.push(map);
                    });
                }
            }
            any if any.is::<SubDataset>() => {
                if let Some(dataset) = any.downcast_mut::<SubDataset>() {
                    dataset.as_mut_vec().into_iter().for_each(|dataset| {
                        v.append(&mut dataset.to_kind_mut_bi_map_vec(kind));
                    });
                }
            }
            _ => {}
        });

        v
    }

    fn to_kind_date_map_vec(&self, kind: MapKind) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let mut v = vec![];

        self.iter().for_each(|(_, any)| {
            if let Some(map) = any.downcast_ref::<DateMap<u8>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<u16>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<u32>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<u64>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<usize>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<f32>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<f64>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<OHLC>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<Date>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<Height>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<DateMap<Timestamp>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyDateMap + Send + Sync))
                }
            } else if let Some(dataset) = any.downcast_ref::<DateRecapDataset<u32>>() {
                dataset.as_vec().into_iter().for_each(|map| {
                    if map.kind() == kind {
                        v.push(map as &(dyn AnyDateMap + Send + Sync))
                    }
                });
            } else if let Some(dataset) = any.downcast_ref::<DateRecapDataset<u64>>() {
                dataset.as_vec().into_iter().for_each(|map| {
                    if map.kind() == kind {
                        v.push(map as &(dyn AnyDateMap + Send + Sync))
                    }
                });
            } else if let Some(dataset) = any.downcast_ref::<DateRecapDataset<f32>>() {
                dataset.as_vec().into_iter().for_each(|map| {
                    if map.kind() == kind {
                        v.push(map as &(dyn AnyDateMap + Send + Sync))
                    }
                });
            } else if let Some(dataset) = any.downcast_ref::<SubDataset>() {
                dataset.as_vec().into_iter().for_each(|dataset| {
                    v.append(&mut dataset.to_kind_date_map_vec(kind));
                });
            } else if let Some(dataset) = any.downcast_ref::<AddressCohortMetadataDataset>() {
                match kind {
                    MapKind::Inserted => dataset.to_inserted_date_map_vec(),
                    MapKind::Computed => dataset.to_computed_date_map_vec(),
                }
                .into_iter()
                .for_each(|map| {
                    v.push(map);
                });
            }
        });

        v
    }

    fn to_kind_mut_date_map_vec(&mut self, kind: MapKind) -> Vec<&mut dyn AnyDateMap> {
        let mut v = vec![];

        self.iter_mut().for_each(|(_, any)| match any {
            any if any.is::<DateMap<u8>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<u8>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<u16>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<u16>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<u32>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<u32>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<u64>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<u64>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<usize>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<usize>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<f32>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<f32>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<f64>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<f64>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<OHLC>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<OHLC>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<Date>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<Date>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<Height>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<Height>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateMap<Timestamp>>() => {
                if let Some(map) = any.downcast_mut::<DateMap<Timestamp>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyDateMap);
                    }
                }
            }
            any if any.is::<DateRecapDataset<u32>>() => {
                if let Some(dataset) = any.downcast_mut::<DateRecapDataset<u32>>() {
                    dataset.as_mut_vec().into_iter().for_each(|map| {
                        if map.kind() == kind {
                            v.push(map as &mut dyn AnyDateMap);
                        }
                    });
                }
            }
            any if any.is::<DateRecapDataset<u64>>() => {
                if let Some(dataset) = any.downcast_mut::<DateRecapDataset<u64>>() {
                    dataset.as_mut_vec().into_iter().for_each(|map| {
                        if map.kind() == kind {
                            v.push(map as &mut dyn AnyDateMap);
                        }
                    });
                }
            }
            any if any.is::<DateRecapDataset<f32>>() => {
                if let Some(dataset) = any.downcast_mut::<DateRecapDataset<f32>>() {
                    dataset.as_mut_vec().into_iter().for_each(|map| {
                        if map.kind() == kind {
                            v.push(map as &mut dyn AnyDateMap);
                        }
                    });
                }
            }
            any if any.is::<SubDataset>() => {
                if let Some(dataset) = any.downcast_mut::<SubDataset>() {
                    dataset.as_mut_vec().into_iter().for_each(|dataset| {
                        v.append(&mut dataset.to_kind_mut_date_map_vec(kind));
                    });
                }
            }
            any if any.is::<AddressCohortMetadataDataset>() => {
                if let Some(dataset) = any.downcast_mut::<AddressCohortMetadataDataset>() {
                    match kind {
                        MapKind::Inserted => dataset.to_inserted_mut_date_map_vec(),
                        MapKind::Computed => dataset.to_computed_mut_date_map_vec(),
                    }
                    .into_iter()
                    .for_each(|map| {
                        v.push(map);
                    });
                }
            }
            _ => {}
        });

        v
    }

    fn to_kind_height_map_vec(&self, kind: MapKind) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        let mut v = vec![];

        self.iter().for_each(|(_, any)| {
            if let Some(map) = any.downcast_ref::<HeightMap<u8>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<u16>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<u32>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<u64>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<usize>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<f32>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<f64>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<OHLC>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<Date>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<Height>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(map) = any.downcast_ref::<HeightMap<Timestamp>>() {
                if map.kind() == kind {
                    v.push(map as &(dyn AnyHeightMap + Send + Sync))
                }
            } else if let Some(dataset) = any.downcast_ref::<SubDataset>() {
                dataset.as_vec().into_iter().for_each(|dataset| {
                    v.append(&mut dataset.to_kind_height_map_vec(kind));
                });
            } else if let Some(dataset) = any.downcast_ref::<AddressCohortMetadataDataset>() {
                match kind {
                    MapKind::Inserted => dataset.to_inserted_height_map_vec(),
                    MapKind::Computed => dataset.to_computed_height_map_vec(),
                }
                .into_iter()
                .for_each(|map| {
                    v.push(map);
                });
            }
        });

        v
    }

    fn to_kind_mut_height_map_vec(&mut self, kind: MapKind) -> Vec<&mut dyn AnyHeightMap> {
        let mut v = vec![];

        self.iter_mut().for_each(|(_, any)| match any {
            any if any.is::<HeightMap<u8>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<u8>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<u16>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<u16>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<u32>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<u32>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<u64>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<u64>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<usize>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<usize>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<f32>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<f32>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<f64>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<f64>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<OHLC>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<OHLC>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<Date>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<Date>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<Height>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<Height>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<HeightMap<Timestamp>>() => {
                if let Some(map) = any.downcast_mut::<HeightMap<Timestamp>>() {
                    if map.kind() == kind {
                        v.push(map as &mut dyn AnyHeightMap);
                    }
                }
            }
            any if any.is::<SubDataset>() => {
                if let Some(dataset) = any.downcast_mut::<SubDataset>() {
                    dataset.as_mut_vec().into_iter().for_each(|dataset| {
                        v.append(&mut dataset.to_kind_mut_height_map_vec(kind));
                    });
                }
            }
            any if any.is::<AddressCohortMetadataDataset>() => {
                if let Some(dataset) = any.downcast_mut::<AddressCohortMetadataDataset>() {
                    match kind {
                        MapKind::Inserted => dataset.to_inserted_mut_height_map_vec(),
                        MapKind::Computed => dataset.to_computed_mut_height_map_vec(),
                    }
                    .into_iter()
                    .for_each(|map| {
                        v.push(map);
                    });
                }
            }
            _ => {}
        });

        v
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.to_kind_bi_map_vec(MapKind::Inserted)
    }

    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.to_kind_height_map_vec(MapKind::Inserted)
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.to_kind_date_map_vec(MapKind::Inserted)
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.to_kind_mut_bi_map_vec(MapKind::Inserted)
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        self.to_kind_mut_height_map_vec(MapKind::Inserted)
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        self.to_kind_mut_date_map_vec(MapKind::Inserted)
    }

    fn to_all_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        let mut vec = self.to_inserted_height_map_vec();

        vec.append(
            &mut self
                .to_inserted_bi_map_vec()
                .iter()
                .map(|bi| bi.get_height())
                .collect_vec(),
        );

        vec
    }

    fn to_all_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let mut vec = self.to_inserted_date_map_vec();

        vec.append(
            &mut self
                .to_inserted_bi_map_vec()
                .iter()
                .map(|bi| bi.get_date())
                .collect_vec(),
        );

        vec
    }

    fn to_all_inserted_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let heights = self
            .to_all_inserted_height_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let dates = self
            .to_all_inserted_date_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        heights.chain(dates).collect_vec()
    }

    #[inline(always)]
    fn should_compute(&self, compute_data: &ComputeData) -> bool {
        compute_data
            .heights
            .last()
            .map_or(false, |height| self.should_compute_height(*height))
            || compute_data
                .dates
                .last()
                .map_or(false, |date| self.should_compute_date(*date))
    }

    #[inline(always)]
    fn should_compute_height(&self, height: Height) -> bool {
        !self.to_all_computed_height_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .computed
                .first_unsafe_height
                .unwrap_or(Height::ZERO)
                <= height
    }

    #[inline(always)]
    fn should_compute_date(&self, date: Date) -> bool {
        !self.to_all_computed_date_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .computed
                .first_unsafe_date
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.to_kind_bi_map_vec(MapKind::Computed)
    }

    fn to_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.to_kind_height_map_vec(MapKind::Computed)
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.to_kind_date_map_vec(MapKind::Computed)
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.to_kind_mut_bi_map_vec(MapKind::Computed)
    }

    fn to_computed_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        self.to_kind_mut_height_map_vec(MapKind::Computed)
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        self.to_kind_mut_date_map_vec(MapKind::Computed)
    }

    fn to_all_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        let mut vec = self.to_computed_height_map_vec();

        vec.append(
            &mut self
                .to_computed_bi_map_vec()
                .iter()
                .map(|bi| bi.get_height())
                .collect_vec(),
        );

        vec
    }

    fn to_all_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let mut vec = self.to_computed_date_map_vec();

        vec.append(
            &mut self
                .to_computed_bi_map_vec()
                .iter()
                .map(|bi| bi.get_date())
                .collect_vec(),
        );

        vec
    }

    fn to_all_computed_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let heights = self
            .to_all_computed_height_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let dates = self
            .to_all_computed_date_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        heights.chain(dates).collect_vec()
    }

    fn to_all_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let mut inserted = self.to_all_inserted_map_vec();

        inserted.append(&mut self.to_all_computed_map_vec());

        inserted
    }

    // #[inline(always)]
    // fn is_empty(&self) -> bool {
    //     self.to_any_map_vec().is_empty()
    // }

    fn pre_export(&mut self) {
        self.to_inserted_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_inserted_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_inserted_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.pre_export())
        });

        self.to_computed_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_computed_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_computed_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.pre_export())
        });
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_all_map_vec()
            .into_par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export() })
    }

    fn post_export(&mut self) {
        self.to_inserted_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_inserted_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_inserted_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.post_export())
        });

        self.to_computed_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_computed_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_computed_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.post_export())
        });
    }

    fn reset_computed(&self) {
        self.to_all_computed_date_map_vec()
            .iter()
            .for_each(|map| map.delete_files());
        self.to_all_computed_height_map_vec()
            .iter()
            .for_each(|map| map.delete_files());
    }
}
