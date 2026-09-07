use super::*;

#[test]
fn block_statistics_preserve_rank_sets_and_invalid_index_handling() {
    let txs: Vec<_> = (0..64u64)
        .map(|i| SnapTx {
            txid: brk_types::Txid::COINBASE,
            fee: Sats::from(i * 100),
            vsize: VSize::new(if i % 7 == 0 { 0 } else { i * 11 }),
            weight: Default::default(),
            size: i * 13,
            chunk_rate: FeeRate::from((i * 7 % 19) as f64),
            parents: Default::default(),
            children: Default::default(),
        })
        .collect();
    let mut block: Vec<_> = (0..txs.len()).rev().map(TxIndex::from).collect();
    block.push(TxIndex::from(999usize));
    let blocks = [block.clone(), block, vec![], vec![TxIndex::from(999usize)]];
    let stats = BlockStats::for_blocks(&blocks, &txs);
    let mut rates: Vec<_> = txs.iter().map(|tx| (tx.chunk_rate, tx.vsize)).collect();
    rates.sort_unstable_by_key(|&(rate, _)| rate);
    for (stat, ranks) in stats.iter().zip([CORE_PERCENTILES, PROJECTED_PERCENTILES]) {
        let expected = ranks.map(|rank| {
            let total: u64 = rates.iter().map(|(_, size)| u64::from(*size)).sum();
            let target = (total as f64 * rank).round() as u64;
            let mut cumulative = 0;
            for &(rate, size) in &rates {
                cumulative += u64::from(size);
                if cumulative >= target {
                    return rate;
                }
            }
            rates.last().unwrap().0
        });
        assert_eq!(stat.fee_range, expected);
        assert_eq!(stat.tx_count, txs.len() as u32);
        assert_eq!(stat.total_fee, txs.iter().map(|tx| tx.fee).sum());
        assert_eq!(stat.total_vsize, txs.iter().map(|tx| tx.vsize).sum());
        assert_eq!(stat.total_size, txs.iter().map(|tx| tx.size).sum::<u64>());
    }
    for stat in &stats[2..] {
        assert_eq!(stat.tx_count, 0);
        assert_eq!(stat.fee_range, [FeeRate::default(); 7]);
    }
}
