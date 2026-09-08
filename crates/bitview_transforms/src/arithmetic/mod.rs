mod days_to_years;
pub use days_to_years::DaysToYears;
mod times_sqrt;
pub use times_sqrt::TimesSqrt;
mod block_count_target;
pub use block_count_target::BlockCountTarget;
mod blocks_to_days_f32;
mod difficulty_to_hash_f64;
mod halve_cents;
mod halve_dollars;
mod halve_sats;
mod halve_sats_to_bitcoin;
mod mask_sats;
mod odds_f64;
mod one_minus_f64;
mod one_minus_ppm;
mod per_second;
mod return_f32_tenths;
mod return_i8;
mod return_u16;
mod stored_u16_to_stored_u64;
mod stored_u64_to_stored_u32;
mod ths_to_phs_f32;
mod v_bytes_to_weight;
mod weight_to_v_size;

pub use blocks_to_days_f32::BlocksToDaysF32;
pub use difficulty_to_hash_f64::DifficultyToHashF64;
pub use halve_cents::HalveCents;
pub use halve_dollars::HalveDollars;
pub use halve_sats::HalveSats;
pub use halve_sats_to_bitcoin::HalveSatsToBitcoin;
pub use mask_sats::MaskSats;
pub use odds_f64::OddsF64;
pub use one_minus_f64::OneMinusF64;
pub use one_minus_ppm::OneMinusPpm;
pub use per_second::PerSecond;
pub use return_f32_tenths::ReturnF32Tenths;
pub use return_i8::ReturnI8;
pub use return_u16::ReturnU16;
pub use stored_u16_to_stored_u64::StoredU16ToStoredU64;
pub use stored_u64_to_stored_u32::StoredU64ToStoredU32;
pub use ths_to_phs_f32::ThsToPhsF32;
pub use v_bytes_to_weight::VBytesToWeight;
pub use weight_to_v_size::WeightToVSize;

#[cfg(test)]
mod tests {
    use super::OddsF64;
    use brk_types::StoredF64;
    use vecdb::UnaryTransform;

    #[test]
    fn odds_are_the_ratio_to_the_complement() {
        assert_eq!(OddsF64::apply(StoredF64::from(0.0)), StoredF64::from(0.0));
        assert_eq!(OddsF64::apply(StoredF64::from(0.5)), StoredF64::from(1.0));
        assert_eq!(OddsF64::apply(StoredF64::from(0.75)), StoredF64::from(3.0));
        assert_eq!(OddsF64::apply(StoredF64::from(1.0)), StoredF64::NAN);
    }
}
