use bitcoin::consensus::Decodable;
use bitcoin::hex::DisplayHex;
use brk_error::{Error, Result};
use brk_reader::Reader;
use brk_types::{
    BlkPosition, BlockExtras, BlockHash, BlockHashPrefix, BlockHeader, BlockInfo, BlockInfoV1,
    BlockPool, FeeRate, Height, Sats, Timestamp, TxIndex, VSize, pools,
};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

const DEFAULT_BLOCK_COUNT: u32 = 10;
const DEFAULT_V1_BLOCK_COUNT: u32 = 15;
const HEADER_SIZE: usize = 80;

impl Query {
    pub fn block(&self, hash: &BlockHash) -> Result<BlockInfo> {
        let height = self.height_by_hash(hash)?;
        self.block_by_height(height)
    }

    pub fn block_by_height(&self, height: Height) -> Result<BlockInfo> {
        let max_height = self.max_height();
        if height > max_height {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }
        self.blocks_range(height.to_usize(), height.to_usize() + 1)?
            .pop()
            .ok_or(Error::NotFound("Block not found".into()))
    }

    pub fn block_by_height_v1(&self, height: Height) -> Result<BlockInfoV1> {
        let max_height = self.max_height();
        if height > max_height {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }
        self.blocks_v1_range(height.to_usize(), height.to_usize() + 1)?
            .pop()
            .ok_or(Error::NotFound("Block not found".into()))
    }

    pub fn block_header_hex(&self, hash: &BlockHash) -> Result<String> {
        let height = self.height_by_hash(hash)?;
        let header = self.read_block_header(height)?;
        Ok(bitcoin::consensus::encode::serialize_hex(&header))
    }

    pub fn block_hash_by_height(&self, height: Height) -> Result<BlockHash> {
        let max_height = self.max_height();
        if height > max_height {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }
        Ok(self.indexer().vecs.blocks.blockhash.read_once(height)?)
    }

    pub fn blocks(&self, start_height: Option<Height>) -> Result<Vec<BlockInfo>> {
        let (begin, end) = self.resolve_block_range(start_height, DEFAULT_BLOCK_COUNT);
        self.blocks_range(begin, end)
    }

    pub fn blocks_v1(&self, start_height: Option<Height>) -> Result<Vec<BlockInfoV1>> {
        let (begin, end) = self.resolve_block_range(start_height, DEFAULT_V1_BLOCK_COUNT);
        self.blocks_v1_range(begin, end)
    }

    // === Range queries (bulk reads) ===

    fn blocks_range(&self, begin: usize, end: usize) -> Result<Vec<BlockInfo>> {
        if begin >= end {
            return Ok(Vec::new());
        }

        let indexer = self.indexer();
        let computer = self.computer();
        let reader = self.reader();

        // Bulk read all indexed data
        let blockhashes = indexer.vecs.blocks.blockhash.collect_range_at(begin, end);
        let difficulties = indexer.vecs.blocks.difficulty.collect_range_at(begin, end);
        let timestamps = indexer.vecs.blocks.timestamp.collect_range_at(begin, end);
        let sizes = indexer.vecs.blocks.total.collect_range_at(begin, end);
        let weights = indexer.vecs.blocks.weight.collect_range_at(begin, end);
        let positions = indexer.vecs.blocks.position.collect_range_at(begin, end);

        // Bulk read tx indexes for tx_count
        let max_height = self.indexed_height();
        let tx_index_end = if end <= max_height.to_usize() {
            end + 1
        } else {
            end
        };
        let first_tx_indexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_range_at(begin, tx_index_end);
        let total_txs = computer.indexes.tx_index.identity.len();

        // Bulk read median time window
        let median_start = begin.saturating_sub(10);
        let median_timestamps: Vec<Timestamp> = indexer
            .vecs
            .blocks
            .timestamp
            .collect_range_at(median_start, end);

        let count = end - begin;
        let mut blocks = Vec::with_capacity(count);

        for i in (0..count).rev() {
            let raw_header = reader.read_raw_bytes(positions[i], HEADER_SIZE)?;
            let header = Self::decode_header(&raw_header)?;

            let tx_count = if i + 1 < first_tx_indexes.len() {
                (first_tx_indexes[i + 1].to_usize() - first_tx_indexes[i].to_usize()) as u32
            } else {
                (total_txs - first_tx_indexes[i].to_usize()) as u32
            };

            let median_time =
                Self::compute_median_time(&median_timestamps, begin + i, median_start);

            blocks.push(BlockInfo {
                id: blockhashes[i].clone(),
                height: Height::from(begin + i),
                version: header.version,
                timestamp: timestamps[i],
                tx_count,
                size: *sizes[i],
                weight: weights[i],
                merkle_root: header.merkle_root,
                previous_block_hash: header.previous_block_hash,
                median_time,
                nonce: header.nonce,
                bits: header.bits,
                difficulty: *difficulties[i],
            });
        }

        Ok(blocks)
    }

    pub(crate) fn blocks_v1_range(&self, begin: usize, end: usize) -> Result<Vec<BlockInfoV1>> {
        if begin >= end {
            return Ok(vec![]);
        }

        let count = end - begin;
        let indexer = self.indexer();
        let computer = self.computer();
        let reader = self.reader();
        let all_pools = pools();

        // Bulk read all indexed data
        let blockhashes = indexer.vecs.blocks.blockhash.collect_range_at(begin, end);
        let difficulties = indexer.vecs.blocks.difficulty.collect_range_at(begin, end);
        let timestamps = indexer.vecs.blocks.timestamp.collect_range_at(begin, end);
        let sizes = indexer.vecs.blocks.total.collect_range_at(begin, end);
        let weights = indexer.vecs.blocks.weight.collect_range_at(begin, end);
        let positions = indexer.vecs.blocks.position.collect_range_at(begin, end);
        let pool_slugs = computer.pools.pool.collect_range_at(begin, end);

        // Bulk read tx indexes
        let max_height = self.indexed_height();
        let tx_index_end = if end <= max_height.to_usize() {
            end + 1
        } else {
            end
        };
        let first_tx_indexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_range_at(begin, tx_index_end);
        let total_txs = computer.indexes.tx_index.identity.len();

        // Bulk read segwit stats
        let segwit_txs = indexer.vecs.blocks.segwit_txs.collect_range_at(begin, end);
        let segwit_sizes = indexer.vecs.blocks.segwit_size.collect_range_at(begin, end);
        let segwit_weights = indexer
            .vecs
            .blocks
            .segwit_weight
            .collect_range_at(begin, end);

        // Bulk read extras data
        let fee_sats = computer
            .mining
            .rewards
            .fees
            .block
            .sats
            .collect_range_at(begin, end);
        let subsidy_sats = computer
            .mining
            .rewards
            .subsidy
            .block
            .sats
            .collect_range_at(begin, end);
        let input_counts = computer.inputs.count.sum.collect_range_at(begin, end);
        let output_counts = computer
            .outputs
            .count
            .total
            .sum
            .collect_range_at(begin, end);
        let utxo_set_sizes = computer
            .outputs
            .count
            .unspent
            .height
            .collect_range_at(begin, end);
        let input_volumes = computer
            .transactions
            .volume
            .transfer_volume
            .block
            .sats
            .collect_range_at(begin, end);
        let output_volumes = computer
            .mining
            .rewards
            .output_volume
            .collect_range_at(begin, end);

        // Bulk read effective fee rate distribution (accounts for CPFP)
        let frd = &computer
            .transactions
            .fees
            .effective_fee_rate
            .distribution
            .block;
        let fr_min = frd.min.height.collect_range_at(begin, end);
        let fr_pct10 = frd.pct10.height.collect_range_at(begin, end);
        let fr_pct25 = frd.pct25.height.collect_range_at(begin, end);
        let fr_median = frd.median.height.collect_range_at(begin, end);
        let fr_pct75 = frd.pct75.height.collect_range_at(begin, end);
        let fr_pct90 = frd.pct90.height.collect_range_at(begin, end);
        let fr_max = frd.max.height.collect_range_at(begin, end);

        // Bulk read fee amount distribution (sats)
        let fad = &computer.transactions.fees.fee.distribution.block;
        let fa_min = fad.min.height.collect_range_at(begin, end);
        let fa_pct10 = fad.pct10.height.collect_range_at(begin, end);
        let fa_pct25 = fad.pct25.height.collect_range_at(begin, end);
        let fa_median = fad.median.height.collect_range_at(begin, end);
        let fa_pct75 = fad.pct75.height.collect_range_at(begin, end);
        let fa_pct90 = fad.pct90.height.collect_range_at(begin, end);
        let fa_max = fad.max.height.collect_range_at(begin, end);

        // Bulk read tx positions range covering all coinbase txs (first tx of each block)
        let tx_pos_begin = first_tx_indexes[0].to_usize();
        let tx_pos_end = first_tx_indexes[count - 1].to_usize() + 1;
        let all_tx_positions = indexer
            .vecs
            .transactions
            .position
            .collect_range_at(tx_pos_begin, tx_pos_end);

        // Bulk read median time window
        let median_start = begin.saturating_sub(10);
        let median_timestamps = indexer
            .vecs
            .blocks
            .timestamp
            .collect_range_at(median_start, end);

        let mut blocks = Vec::with_capacity(count);

        for i in (0..count).rev() {
            let raw_header = reader.read_raw_bytes(positions[i], HEADER_SIZE)?;
            let header = Self::decode_header(&raw_header)?;

            let tx_count = if i + 1 < first_tx_indexes.len() {
                (first_tx_indexes[i + 1].to_usize() - first_tx_indexes[i].to_usize()) as u32
            } else {
                (total_txs - first_tx_indexes[i].to_usize()) as u32
            };

            let weight = weights[i];
            let size = *sizes[i];
            let total_fees = fee_sats[i];
            let subsidy = subsidy_sats[i];
            let total_inputs = (*input_counts[i]).saturating_sub(1);
            let total_outputs = *output_counts[i];
            let vsize = weight.to_vbytes_ceil();
            let total_fees_u64 = u64::from(total_fees);
            let non_coinbase = tx_count.saturating_sub(1) as u64;

            let pool_slug = pool_slugs[i];
            let pool = all_pools.get(pool_slug);

            let (
                coinbase_raw,
                coinbase_address,
                coinbase_addresses,
                coinbase_signature,
                coinbase_signature_ascii,
            ) = Self::parse_coinbase_tx(
                reader,
                all_tx_positions[first_tx_indexes[i].to_usize() - tx_pos_begin],
            );

            let median_time =
                Self::compute_median_time(&median_timestamps, begin + i, median_start);

            let info = BlockInfo {
                id: blockhashes[i].clone(),
                height: Height::from(begin + i),
                version: header.version,
                timestamp: timestamps[i],
                tx_count,
                size,
                weight,
                merkle_root: header.merkle_root,
                previous_block_hash: header.previous_block_hash,
                median_time,
                nonce: header.nonce,
                bits: header.bits,
                difficulty: *difficulties[i],
            };

            let total_input_amt = input_volumes[i];
            let total_output_amt = output_volumes[i];

            let extras = BlockExtras {
                total_fees,
                median_fee: fr_median[i],
                fee_range: [
                    fr_min[i],
                    fr_pct10[i],
                    fr_pct25[i],
                    fr_median[i],
                    fr_pct75[i],
                    fr_pct90[i],
                    fr_max[i],
                ],
                reward: subsidy + total_fees,
                pool: BlockPool {
                    id: pool.unique_id(),
                    name: pool.name.to_string(),
                    slug: pool_slug,
                    miner_names: None,
                },
                avg_fee: Sats::from(if non_coinbase > 0 {
                    total_fees_u64 / non_coinbase
                } else {
                    0
                }),
                avg_fee_rate: FeeRate::from((total_fees, VSize::from(vsize))),
                coinbase_raw,
                coinbase_address,
                coinbase_addresses,
                coinbase_signature,
                coinbase_signature_ascii,
                avg_tx_size: if tx_count > 0 {
                    size as f64 / tx_count as f64
                } else {
                    0.0
                },
                total_inputs,
                total_outputs,
                total_output_amt,
                median_fee_amt: fa_median[i],
                fee_percentiles: [
                    fa_min[i],
                    fa_pct10[i],
                    fa_pct25[i],
                    fa_median[i],
                    fa_pct75[i],
                    fa_pct90[i],
                    fa_max[i],
                ],
                segwit_total_txs: *segwit_txs[i],
                segwit_total_size: *segwit_sizes[i],
                segwit_total_weight: segwit_weights[i],
                header: raw_header.to_lower_hex_string(),
                utxo_set_change: total_outputs as i64 - total_inputs as i64,
                utxo_set_size: *utxo_set_sizes[i],
                total_input_amt,
                virtual_size: vsize as f64,
            };

            blocks.push(BlockInfoV1 { info, extras });
        }

        Ok(blocks)
    }

    // === Helper methods ===

    pub fn height_by_hash(&self, hash: &BlockHash) -> Result<Height> {
        let indexer = self.indexer();
        let prefix = BlockHashPrefix::from(hash);
        indexer
            .stores
            .blockhash_prefix_to_height
            .get(&prefix)?
            .map(|h| *h)
            .ok_or(Error::NotFound("Block not found".into()))
    }

    pub fn read_block_header(&self, height: Height) -> Result<bitcoin::block::Header> {
        let position = self
            .indexer()
            .vecs
            .blocks
            .position
            .collect_one(height)
            .unwrap();
        let raw = self.reader().read_raw_bytes(position, HEADER_SIZE)?;
        bitcoin::block::Header::consensus_decode(&mut raw.as_slice())
            .map_err(|_| Error::Internal("Failed to decode block header"))
    }

    fn max_height(&self) -> Height {
        Height::from(self.indexer().vecs.blocks.blockhash.len().saturating_sub(1))
    }

    fn resolve_block_range(&self, start_height: Option<Height>, count: u32) -> (usize, usize) {
        let max_height = self.height();
        let start = start_height.unwrap_or(max_height).min(max_height);
        let start_u32: u32 = start.into();
        let count = count.min(start_u32 + 1) as usize;
        let end = start_u32 as usize + 1;
        let begin = end - count;
        (begin, end)
    }

    fn decode_header(bytes: &[u8]) -> Result<BlockHeader> {
        let raw = bitcoin::block::Header::consensus_decode(&mut &bytes[..])
            .map_err(|_| Error::Internal("Failed to decode block header"))?;
        Ok(BlockHeader::from(raw))
    }

    fn compute_median_time(
        all_timestamps: &[Timestamp],
        height: usize,
        window_start: usize,
    ) -> Timestamp {
        let rel_start = height.saturating_sub(10) - window_start;
        let rel_end = height + 1 - window_start;
        let mut sorted: Vec<usize> = all_timestamps[rel_start..rel_end]
            .iter()
            .map(|t| usize::from(*t))
            .collect();
        sorted.sort_unstable();
        Timestamp::from(sorted[sorted.len() / 2])
    }

    fn parse_coinbase_tx(
        reader: &Reader,
        position: BlkPosition,
    ) -> (String, Option<String>, Vec<String>, String, String) {
        let raw_bytes = match reader.read_raw_bytes(position, 1000) {
            Ok(bytes) => bytes,
            Err(_) => return (String::new(), None, vec![], String::new(), String::new()),
        };

        let tx = match bitcoin::Transaction::consensus_decode(&mut raw_bytes.as_slice()) {
            Ok(tx) => tx,
            Err(_) => return (String::new(), None, vec![], String::new(), String::new()),
        };

        let coinbase_raw = tx
            .input
            .first()
            .map(|input| input.script_sig.as_bytes().to_lower_hex_string())
            .unwrap_or_default();

        let coinbase_signature_ascii = tx
            .input
            .first()
            .map(|input| {
                input
                    .script_sig
                    .as_bytes()
                    .iter()
                    .map(|&b| b as char)
                    .collect::<String>()
            })
            .unwrap_or_default();

        let coinbase_addresses: Vec<String> = tx
            .output
            .iter()
            .filter_map(|output| {
                bitcoin::Address::from_script(&output.script_pubkey, bitcoin::Network::Bitcoin)
                    .ok()
                    .map(|a| a.to_string())
            })
            .collect();
        let coinbase_address = coinbase_addresses.first().cloned();

        let coinbase_signature = tx
            .output
            .first()
            .map(|output| output.script_pubkey.to_asm_string())
            .unwrap_or_default();

        (
            coinbase_raw,
            coinbase_address,
            coinbase_addresses,
            coinbase_signature,
            coinbase_signature_ascii,
        )
    }
}
