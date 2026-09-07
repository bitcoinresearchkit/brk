use brk_error::Result;

use bitview_compute::{RatioDiffF32, TARGET_BLOCKS_PER_DAY_F64};
use bitview_plugin_blocks::ONE_TERA_HASH;
use bitview_plugin_indexer::Indexer;
use brk_exit::Exit;
use brk_types::{Dollars, Height, PartsPerMillionSigned32, Sats, StoredF32, StoredF64};
use vecdb::ReadableVec;

use super::Vecs;
#[inline]
fn estimated_network_hash_rate(block_count_24h: f64, difficulty_hash_rate: f64) -> f64 {
    (block_count_24h / TARGET_BLOCKS_PER_DAY_F64) * difficulty_hash_rate
}

#[inline]
fn reward_per_ths(reward_24h: f64, hash_rate: f64) -> StoredF32 {
    let hash_rate_ths = hash_rate / ONE_TERA_HASH;
    if hash_rate_ths == 0.0 {
        StoredF32::NAN
    } else {
        StoredF32::from(reward_24h / hash_rate_ths)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    count_vecs: &bitview_plugin_blocks::CountVecs,
    lookback: &bitview_plugin_blocks::LookbackVecs,
    difficulty_vecs: &bitview_plugin_blocks::DifficultyVecs,
    coinbase_sats_24h_sum: &impl ReadableVec<Height, Sats>,
    coinbase_usd_24h_sum: &impl ReadableVec<Height, Dollars>,
    exit: &Exit,
) -> Result<()> {
    let starting_height = indexer.safe_lengths().height;

    vecs.rate.base.height.compute_transform2(
        starting_height,
        &count_vecs.total.sum._24h.height,
        &difficulty_vecs.hashrate.height,
        |(i, block_count_sum, difficulty_as_hash, ..)| {
            (
                i,
                StoredF64::from(estimated_network_hash_rate(
                    f64::from(block_count_sum),
                    f64::from(difficulty_as_hash),
                )),
            )
        },
        exit,
    )?;

    let hash_rate = &vecs.rate.base.height;
    for (sma, window) in [
        (&mut vecs.rate.sma._1w.height, lookback._1w.lazy()),
        (&mut vecs.rate.sma._1m.height, lookback._1m.lazy()),
        (&mut vecs.rate.sma._2m.height, &lookback._2m),
        (&mut vecs.rate.sma._1y.height, lookback._1y.lazy()),
    ] {
        sma.compute_rolling_average(starting_height, window, hash_rate, exit)?;
    }

    vecs.rate
        .ath
        .height
        .compute_all_time_high(starting_height, &vecs.rate.base.height, exit)?;

    vecs.rate.drawdown.compute_drawdown(
        starting_height,
        &vecs.rate.base.height,
        &vecs.rate.ath.height,
        exit,
    )?;

    vecs.price.ths.height.compute_transform2(
        starting_height,
        coinbase_usd_24h_sum,
        &vecs.rate.base.height,
        |(i, coinbase_sum, hashrate, ..)| (i, reward_per_ths(f64::from(coinbase_sum), *hashrate)),
        exit,
    )?;

    vecs.value.ths.height.compute_transform2(
        starting_height,
        coinbase_sats_24h_sum,
        &vecs.rate.base.height,
        |(i, coinbase_sum, hashrate, ..)| (i, reward_per_ths(f64::from(coinbase_sum), *hashrate)),
        exit,
    )?;

    for (min_vec, src_vec) in [
        (&mut vecs.price.ths_min.height, &vecs.price.ths.height),
        (&mut vecs.value.ths_min.height, &vecs.value.ths.height),
    ] {
        min_vec.compute_all_time_low_(starting_height, src_vec, exit, true)?;
    }

    vecs.price
        .rebound
        .compute_binary::<StoredF32, StoredF32, RatioDiffF32<PartsPerMillionSigned32>>(
            starting_height,
            &vecs.price.phs.height,
            &vecs.price.phs_min.height,
            exit,
        )?;

    vecs.value
        .rebound
        .compute_binary::<StoredF32, StoredF32, RatioDiffF32<PartsPerMillionSigned32>>(
            starting_height,
            &vecs.value.phs.height,
            &vecs.value.phs_min.height,
            exit,
        )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use brk_types::{PartsPerMillionSigned32, PartsPerMillionSigned64, StoredF32};
    use vecdb::{BinaryTransform, UnaryTransform};

    use super::{estimated_network_hash_rate, reward_per_ths};
    use bitview_compute::{RatioDiffF32, ThsToPhsF32};

    #[test]
    fn network_hash_rate_scales_with_trailing_block_count() {
        let difficulty_hash_rate = 600_000_000_000_000_000.0;
        assert_eq!(
            estimated_network_hash_rate(144.0, difficulty_hash_rate),
            difficulty_hash_rate
        );
        assert_eq!(
            estimated_network_hash_rate(72.0, difficulty_hash_rate),
            difficulty_hash_rate / 2.0
        );
    }

    #[test]
    fn hash_revenue_units_and_rebound_are_exact() {
        let ths = reward_per_ths(1_000.0, 100.0 * 1_000_000_000_000.0);
        assert_eq!(*ths, 10.0);

        let phs = ThsToPhsF32::apply(ths);
        assert_eq!(*phs, 10_000.0);

        let rebound = RatioDiffF32::<PartsPerMillionSigned32>::apply(
            StoredF32::from(15.0),
            StoredF32::from(10.0),
        );
        assert_eq!(rebound.inner(), 500_000);

        assert!(reward_per_ths(1_000.0, 0.0).is_nan());
    }

    #[test]
    fn narrowed_rebounds_match_signed_64_ppm() {
        for (current, minimum) in [
            (15.0, 10.0),
            (1.0, 3.0),
            (500.0, 1.0),
            (1.0, 0.0),
            (f32::NAN, 1.0),
        ] {
            let current = StoredF32::from(current);
            let minimum = StoredF32::from(minimum);
            let actual = RatioDiffF32::<PartsPerMillionSigned32>::apply(current, minimum);
            let expected = RatioDiffF32::<PartsPerMillionSigned64>::apply(current, minimum);
            if expected.is_nan() {
                assert!(actual.is_nan());
            } else {
                assert_eq!(i64::from(actual.inner()), expected.inner());
            }
        }
    }
}
