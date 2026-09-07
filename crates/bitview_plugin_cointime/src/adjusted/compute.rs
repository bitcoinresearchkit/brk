use brk_error::Result;

use bitview_plugin_indexer::Indexer;
use brk_exit::Exit;
use brk_types::{Height, PartsPerMillionSigned32, PartsPerMillionSigned64, StoredF64};
use vecdb::ReadableVec;

use super::super::activity;
use super::Vecs;

#[inline]
fn adjusted_inflation(
    active_to_vaulted: StoredF64,
    inflation: PartsPerMillionSigned64,
) -> PartsPerMillionSigned32 {
    PartsPerMillionSigned32::from(f64::from(active_to_vaulted) * f64::from(inflation))
}

pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    inflation_rate: &impl ReadableVec<Height, PartsPerMillionSigned64>,
    velocity_native: &impl ReadableVec<Height, StoredF64>,
    velocity_fiat: &impl ReadableVec<Height, StoredF64>,
    activity: &activity::Vecs,
    exit: &Exit,
) -> Result<()> {
    let starting_height = indexer.safe_lengths().height;

    vecs.inflation_rate.ppm.height.compute_transform2(
        starting_height,
        &activity.ratio.height,
        inflation_rate,
        |(h, a2vr, inflation, ..)| (h, adjusted_inflation(a2vr, inflation)),
        exit,
    )?;

    vecs.tx_velocity_native.height.compute_multiply(
        starting_height,
        &activity.ratio.height,
        velocity_native,
        exit,
    )?;

    vecs.tx_velocity_fiat.height.compute_multiply(
        starting_height,
        &activity.ratio.height,
        velocity_fiat,
        exit,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjusted_inflation_retains_ppm_precision_and_nan() {
        for (activity, inflation) in [(2.5, 0.02), (0.3, -0.012345), (1.0, 200.0)] {
            let inflation = PartsPerMillionSigned64::from(inflation);
            let expected = PartsPerMillionSigned64::from(activity * f64::from(inflation));
            let actual = adjusted_inflation(StoredF64::from(activity), inflation);
            assert_eq!(i64::from(actual.inner()), expected.inner());
        }
        assert!(adjusted_inflation(StoredF64::from(1.0), PartsPerMillionSigned64::NAN).is_nan());
        assert!(adjusted_inflation(StoredF64::NAN, PartsPerMillionSigned64::ONE).is_nan());
    }
}
