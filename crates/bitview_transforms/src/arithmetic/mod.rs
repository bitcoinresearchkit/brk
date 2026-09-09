mod days_to_years;
pub use days_to_years::DaysToYears;
mod times_sqrt;
pub use times_sqrt::TimesSqrt;
mod block_count_target;
pub use block_count_target::BlockCountTarget;
mod blocks_to_days_f32;
mod difficulty_to_hash_f64;
mod halve_dollars;
mod halve_sats_to_bitcoin;
mod mask_sats;
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
pub use halve_dollars::HalveDollars;
pub use halve_sats_to_bitcoin::HalveSatsToBitcoin;
pub use mask_sats::MaskSats;
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
    use super::{HalveDollars, HalveSatsToBitcoin};
    use brk_types::{Bitcoin, Cents, Dollars, Sats};
    use vecdb::{Halve, UnaryTransform};

    #[test]
    fn integer_halves_keep_rounding_and_missing_values() {
        for value in [0u64, 1, 3, 2_100_000_000_000_001] {
            let sats = Sats::from(value);
            assert_eq!(Halve::apply(sats), Sats::from(value / 2));
            assert_eq!(Halve::apply(Cents::from(value)), Cents::from(value / 2));
            assert_eq!(HalveSatsToBitcoin::apply(sats), Bitcoin::from(sats / 2));
        }
        assert!(Halve::apply(Cents::NAN).is_nan());
    }

    #[test]
    fn dollar_halves_preserve_sub_cent_precision() {
        for value in [0.0, 0.01, -0.01, 1.2345] {
            assert_eq!(
                HalveDollars::apply(Dollars::from(value)),
                Dollars::from(value / 2.0)
            );
        }
        assert!(HalveDollars::apply(Dollars::NAN).is_nan());
    }
}
