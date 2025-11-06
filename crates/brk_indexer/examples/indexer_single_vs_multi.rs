use std::{
    fs,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::TxInIndex;
use rayon::prelude::*;
use vecdb::{AnyVec, GenericStoredVec, StoredIndex};

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;

    let indexer = Indexer::forced_import(&outputs_dir)?;
    let vecs = indexer.vecs;

    let output_len = vecs.txoutindex_to_value.len();
    let input_len = vecs.txinindex_to_outpoint.len();
    dbg!(output_len, input_len);

    // Simulate processing blocks
    const NUM_BLOCKS: usize = 10_000;
    const OUTPUTS_PER_BLOCK: usize = 5_000;
    const INPUTS_PER_BLOCK: usize = 5_000;
    const OUTPUT_START_OFFSET: usize = 2_000_000_000;
    const INPUT_START_OFFSET: usize = 2_000_000_000;
    const NUM_RUNS: usize = 3;

    println!(
        "\n=== Running {} iterations of {} blocks ===",
        NUM_RUNS, NUM_BLOCKS
    );
    println!("  {} outputs per block", OUTPUTS_PER_BLOCK);
    println!("  {} inputs per block\n", INPUTS_PER_BLOCK);

    // Store all run times
    let mut method1_times = Vec::new();
    let mut method2_times = Vec::new();
    let mut method4_times = Vec::new();
    let mut method5_times = Vec::new();
    let mut method6_times = Vec::new();
    let mut method7_times = Vec::new();
    let mut method8_times = Vec::new();

    for run in 0..NUM_RUNS {
        println!("--- Run {}/{} ---", run + 1, NUM_RUNS);

        // Randomize order for this run
        let order = match run % 4 {
            0 => vec![1, 2, 4, 5, 6, 7, 8],
            1 => vec![8, 7, 6, 5, 4, 2, 1],
            2 => vec![2, 5, 8, 1, 7, 4, 6],
            _ => vec![6, 4, 7, 1, 8, 5, 2],
        };

        let mut run_times = [Duration::ZERO; 7];

        for &method in &order {
            match method {
                1 => {
                    let time = run_method1(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    );
                    run_times[0] = time;
                }
                2 => {
                    let time = run_method2(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    )?;
                    run_times[1] = time;
                }
                4 => {
                    let time = run_method4(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    )?;
                    run_times[2] = time;
                }
                5 => {
                    let time = run_method5(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    );
                    run_times[3] = time;
                }
                6 => {
                    let time = run_method6(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    )?;
                    run_times[4] = time;
                }
                7 => {
                    let time = run_method7(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    );
                    run_times[5] = time;
                }
                8 => {
                    let time = run_method8(
                        &vecs,
                        NUM_BLOCKS,
                        OUTPUTS_PER_BLOCK,
                        INPUTS_PER_BLOCK,
                        OUTPUT_START_OFFSET,
                        INPUT_START_OFFSET,
                    );
                    run_times[6] = time;
                }
                _ => unreachable!(),
            }
        }

        method1_times.push(run_times[0]);
        method2_times.push(run_times[1]);
        method4_times.push(run_times[2]);
        method5_times.push(run_times[3]);
        method6_times.push(run_times[4]);
        method7_times.push(run_times[5]);
        method8_times.push(run_times[6]);

        println!("  Method 1: {:?}", run_times[0]);
        println!("  Method 2: {:?}", run_times[1]);
        println!("  Method 4: {:?}", run_times[2]);
        println!("  Method 5: {:?}", run_times[3]);
        println!("  Method 6: {:?}", run_times[4]);
        println!("  Method 7: {:?}", run_times[5]);
        println!("  Method 8: {:?}", run_times[6]);
        println!();
    }

    // Calculate statistics
    println!("\n=== Statistics over {} runs ===\n", NUM_RUNS);

    let methods = vec![
        ("Method 1 (Parallel Interleaved)", &method1_times),
        (
            "Method 2 (Sequential Read + Parallel Process)",
            &method2_times,
        ),
        (
            "Method 4 (Parallel Sequential Reads + Parallel Process)",
            &method4_times,
        ),
        ("Method 5 (Chunked Parallel)", &method5_times),
        ("Method 6 (Prefetch)", &method6_times),
        ("Method 7 (Reuse Readers)", &method7_times),
        ("Method 8 (Bulk Processing)", &method8_times),
    ];

    for (name, times) in &methods {
        let avg = times.iter().sum::<Duration>() / times.len() as u32;
        let min = times.iter().min().unwrap();
        let max = times.iter().max().unwrap();

        println!("{}:", name);
        println!("  Average: {:?}", avg);
        println!("  Min: {:?}", min);
        println!("  Max: {:?}", max);
        println!("  Std dev: {:?}", calculate_stddev(times));
        println!();
    }

    // Find overall winner based on average
    let averages: Vec<_> = methods
        .iter()
        .map(|(name, times)| {
            let avg = times.iter().sum::<Duration>() / times.len() as u32;
            (*name, avg)
        })
        .collect();

    let fastest = averages.iter().min_by_key(|(_, t)| t).unwrap();
    println!(
        "=== Winner (by average): {} - {:?} ===\n",
        fastest.0, fastest.1
    );

    for (name, time) in &averages {
        if time != &fastest.1 {
            let diff = time.as_secs_f64() / fastest.1.as_secs_f64();
            println!("{} is {:.2}x slower", name, diff);
        }
    }

    Ok(())
}

fn run_method1(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Duration {
    let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();
    let txoutindex_to_outputtype_reader = vecs.txoutindex_to_outputtype.create_reader();
    let txoutindex_to_typeindex_reader = vecs.txoutindex_to_typeindex.create_reader();
    let txinindex_to_outpoint_reader = vecs.txinindex_to_outpoint.create_reader();
    let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();

    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        // Process outputs
        let block_start = output_start_offset + (block_idx * outputs_per_block);

        let _outputs: Vec<_> = (block_start..(block_start + outputs_per_block))
            .into_par_iter()
            .map(|i| {
                (
                    vecs.txoutindex_to_value
                        .read_at_unwrap(i, &txoutindex_to_value_reader),
                    vecs.txoutindex_to_outputtype
                        .read_at_unwrap(i, &txoutindex_to_outputtype_reader),
                    vecs.txoutindex_to_typeindex
                        .read_at_unwrap(i, &txoutindex_to_typeindex_reader),
                )
            })
            .collect();

        // Process inputs
        let input_block_start = input_start_offset + (block_idx * inputs_per_block);

        let input_sum: u64 = (input_block_start..(input_block_start + inputs_per_block))
            .into_par_iter()
            .filter_map(|i| {
                let outpoint = vecs
                    .txinindex_to_outpoint
                    .read_at_unwrap(i, &txinindex_to_outpoint_reader);

                if outpoint.is_coinbase() {
                    return None;
                }

                let first_txoutindex = vecs.txindex_to_first_txoutindex.read_at_unwrap(
                    outpoint.txindex().to_usize(),
                    &txindex_to_first_txoutindex_reader,
                );
                let txoutindex = first_txoutindex.to_usize() + usize::from(outpoint.vout());
                let value = vecs
                    .txoutindex_to_value
                    .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                Some(u64::from(value))
            })
            .sum();

        std::hint::black_box(input_sum);
    }

    start_time.elapsed()
}

fn run_method2(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Result<Duration> {
    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        // Process outputs
        let block_start = brk_types::TxOutIndex::new(
            (output_start_offset + (block_idx * outputs_per_block)) as u64,
        );

        let values: Vec<_> = vecs
            .txoutindex_to_value
            .iter()?
            .skip(block_start.to_usize())
            .take(outputs_per_block)
            .collect();

        let output_types: Vec<_> = vecs
            .txoutindex_to_outputtype
            .iter()?
            .skip(block_start.to_usize())
            .take(outputs_per_block)
            .collect();

        let typeindexes: Vec<_> = vecs
            .txoutindex_to_typeindex
            .iter()?
            .skip(block_start.to_usize())
            .take(outputs_per_block)
            .collect();

        let _outputs: Vec<_> = (0..outputs_per_block)
            .into_par_iter()
            .map(|i| (values[i], output_types[i], typeindexes[i]))
            .collect();

        // Process inputs
        let input_block_start =
            TxInIndex::new((input_start_offset + (block_idx * inputs_per_block)) as u64);

        let outpoints: Vec<_> = vecs
            .txinindex_to_outpoint
            .iter()?
            .skip(input_block_start.to_usize())
            .take(inputs_per_block)
            .collect();

        let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();
        let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();

        let input_sum: u64 = (0..outpoints.len())
            .into_par_iter()
            .filter_map(|i| {
                let outpoint = outpoints[i];

                if outpoint.is_coinbase() {
                    return None;
                }

                let first_txoutindex = vecs.txindex_to_first_txoutindex.read_at_unwrap(
                    outpoint.txindex().to_usize(),
                    &txindex_to_first_txoutindex_reader,
                );
                let txoutindex = first_txoutindex.to_usize() + usize::from(outpoint.vout());
                let value = vecs
                    .txoutindex_to_value
                    .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                Some(u64::from(value))
            })
            .sum();

        std::hint::black_box(input_sum);
    }

    Ok(start_time.elapsed())
}

fn run_method4(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Result<Duration> {
    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        // Process outputs with parallel reads
        let block_start = brk_types::TxOutIndex::new(
            (output_start_offset + (block_idx * outputs_per_block)) as u64,
        );

        let (values, output_types, typeindexes) = thread::scope(|s| -> Result<_> {
            let h1 = s.spawn(|| -> Result<_> {
                Ok(vecs
                    .txoutindex_to_value
                    .iter()?
                    .skip(block_start.to_usize())
                    .take(outputs_per_block)
                    .collect::<Vec<_>>())
            });

            let h2 = s.spawn(|| -> Result<_> {
                Ok(vecs
                    .txoutindex_to_outputtype
                    .iter()?
                    .skip(block_start.to_usize())
                    .take(outputs_per_block)
                    .collect::<Vec<_>>())
            });

            let h3 = s.spawn(|| -> Result<_> {
                Ok(vecs
                    .txoutindex_to_typeindex
                    .iter()?
                    .skip(block_start.to_usize())
                    .take(outputs_per_block)
                    .collect::<Vec<_>>())
            });

            Ok((
                h1.join().unwrap()?,
                h2.join().unwrap()?,
                h3.join().unwrap()?,
            ))
        })?;

        let _outputs: Vec<_> = (0..outputs_per_block)
            .into_par_iter()
            .map(|i| (values[i], output_types[i], typeindexes[i]))
            .collect();

        // Process inputs
        let input_block_start =
            TxInIndex::new((input_start_offset + (block_idx * inputs_per_block)) as u64);

        let outpoints: Vec<_> = vecs
            .txinindex_to_outpoint
            .iter()?
            .skip(input_block_start.to_usize())
            .take(inputs_per_block)
            .collect();

        let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();
        let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();

        let input_sum: u64 = (0..outpoints.len())
            .into_par_iter()
            .filter_map(|i| {
                let outpoint = outpoints[i];

                if outpoint.is_coinbase() {
                    return None;
                }

                let first_txoutindex = vecs.txindex_to_first_txoutindex.read_at_unwrap(
                    outpoint.txindex().to_usize(),
                    &txindex_to_first_txoutindex_reader,
                );
                let txoutindex = first_txoutindex.to_usize() + usize::from(outpoint.vout());
                let value = vecs
                    .txoutindex_to_value
                    .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                Some(u64::from(value))
            })
            .sum();

        std::hint::black_box(input_sum);
    }

    Ok(start_time.elapsed())
}

fn run_method5(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Duration {
    let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();
    let txoutindex_to_outputtype_reader = vecs.txoutindex_to_outputtype.create_reader();
    let txoutindex_to_typeindex_reader = vecs.txoutindex_to_typeindex.create_reader();
    let txinindex_to_outpoint_reader = vecs.txinindex_to_outpoint.create_reader();
    let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();

    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        // Process outputs with larger chunks
        let block_start = output_start_offset + (block_idx * outputs_per_block);

        let _outputs: Vec<_> = (block_start..(block_start + outputs_per_block))
            .into_par_iter()
            .with_min_len(500) // Larger chunks
            .map(|i| {
                (
                    vecs.txoutindex_to_value
                        .read_at_unwrap(i, &txoutindex_to_value_reader),
                    vecs.txoutindex_to_outputtype
                        .read_at_unwrap(i, &txoutindex_to_outputtype_reader),
                    vecs.txoutindex_to_typeindex
                        .read_at_unwrap(i, &txoutindex_to_typeindex_reader),
                )
            })
            .collect();

        // Process inputs with larger chunks
        let input_block_start = input_start_offset + (block_idx * inputs_per_block);

        let input_sum: u64 = (input_block_start..(input_block_start + inputs_per_block))
            .into_par_iter()
            .with_min_len(500) // Larger chunks
            .filter_map(|i| {
                let outpoint = vecs
                    .txinindex_to_outpoint
                    .read_at_unwrap(i, &txinindex_to_outpoint_reader);

                if outpoint.is_coinbase() {
                    return None;
                }

                let first_txoutindex = vecs.txindex_to_first_txoutindex.read_at_unwrap(
                    outpoint.txindex().to_usize(),
                    &txindex_to_first_txoutindex_reader,
                );
                let txoutindex = first_txoutindex.to_usize() + usize::from(outpoint.vout());
                let value = vecs
                    .txoutindex_to_value
                    .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                Some(u64::from(value))
            })
            .sum();

        std::hint::black_box(input_sum);
    }

    start_time.elapsed()
}

fn run_method6(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Result<Duration> {
    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        // Read outputs sequentially
        let block_start = brk_types::TxOutIndex::new(
            (output_start_offset + (block_idx * outputs_per_block)) as u64,
        );

        let values: Vec<_> = vecs
            .txoutindex_to_value
            .iter()?
            .skip(block_start.to_usize())
            .take(outputs_per_block)
            .collect();

        let output_types: Vec<_> = vecs
            .txoutindex_to_outputtype
            .iter()?
            .skip(block_start.to_usize())
            .take(outputs_per_block)
            .collect();

        let typeindexes: Vec<_> = vecs
            .txoutindex_to_typeindex
            .iter()?
            .skip(block_start.to_usize())
            .take(outputs_per_block)
            .collect();

        // Read inputs sequentially
        let input_block_start =
            TxInIndex::new((input_start_offset + (block_idx * inputs_per_block)) as u64);

        let outpoints: Vec<_> = vecs
            .txinindex_to_outpoint
            .iter()?
            .skip(input_block_start.to_usize())
            .take(inputs_per_block)
            .collect();

        let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();
        let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();

        // Prefetch all first_txoutindexes in parallel
        let first_txoutindexes: Vec<Option<_>> =
            outpoints
                .par_iter()
                .map(|op| {
                    if op.is_coinbase() {
                        return None;
                    }
                    Some(vecs.txindex_to_first_txoutindex.read_at_unwrap(
                        op.txindex().to_usize(),
                        &txindex_to_first_txoutindex_reader,
                    ))
                })
                .collect();

        // Then read values in parallel
        let input_sum: u64 = outpoints
            .par_iter()
            .zip(first_txoutindexes.par_iter())
            .filter_map(|(op, first_opt)| {
                let first_txoutindex = first_opt.as_ref()?;
                let txoutindex = first_txoutindex.to_usize() + usize::from(op.vout());
                let value = vecs
                    .txoutindex_to_value
                    .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                Some(u64::from(value))
            })
            .sum();

        let _outputs: Vec<_> = (0..outputs_per_block)
            .into_par_iter()
            .map(|i| (values[i], output_types[i], typeindexes[i]))
            .collect();

        std::hint::black_box(input_sum);
    }

    Ok(start_time.elapsed())
}

fn run_method7(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Duration {
    // Create readers ONCE outside loop
    let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();
    let txoutindex_to_outputtype_reader = vecs.txoutindex_to_outputtype.create_reader();
    let txoutindex_to_typeindex_reader = vecs.txoutindex_to_typeindex.create_reader();
    let txinindex_to_outpoint_reader = vecs.txinindex_to_outpoint.create_reader();
    let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();

    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        let block_start = output_start_offset + (block_idx * outputs_per_block);

        let _outputs: Vec<_> = (block_start..(block_start + outputs_per_block))
            .into_par_iter()
            .map(|i| {
                (
                    vecs.txoutindex_to_value
                        .read_at_unwrap(i, &txoutindex_to_value_reader),
                    vecs.txoutindex_to_outputtype
                        .read_at_unwrap(i, &txoutindex_to_outputtype_reader),
                    vecs.txoutindex_to_typeindex
                        .read_at_unwrap(i, &txoutindex_to_typeindex_reader),
                )
            })
            .collect();

        let input_block_start = input_start_offset + (block_idx * inputs_per_block);

        let input_sum: u64 = (input_block_start..(input_block_start + inputs_per_block))
            .into_par_iter()
            .filter_map(|i| {
                let outpoint = vecs
                    .txinindex_to_outpoint
                    .read_at_unwrap(i, &txinindex_to_outpoint_reader);

                if outpoint.is_coinbase() {
                    return None;
                }

                let first_txoutindex = vecs.txindex_to_first_txoutindex.read_at_unwrap(
                    outpoint.txindex().to_usize(),
                    &txindex_to_first_txoutindex_reader,
                );
                let txoutindex = first_txoutindex.to_usize() + usize::from(outpoint.vout());
                let value = vecs
                    .txoutindex_to_value
                    .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                Some(u64::from(value))
            })
            .sum();

        std::hint::black_box(input_sum);
    }

    start_time.elapsed()
}

fn run_method8(
    vecs: &brk_indexer::Vecs,
    num_blocks: usize,
    outputs_per_block: usize,
    inputs_per_block: usize,
    output_start_offset: usize,
    input_start_offset: usize,
) -> Duration {
    let txoutindex_to_value_reader = vecs.txoutindex_to_value.create_reader();
    let txoutindex_to_outputtype_reader = vecs.txoutindex_to_outputtype.create_reader();
    let txoutindex_to_typeindex_reader = vecs.txoutindex_to_typeindex.create_reader();
    let txinindex_to_outpoint_reader = vecs.txinindex_to_outpoint.create_reader();
    let txindex_to_first_txoutindex_reader = vecs.txindex_to_first_txoutindex.create_reader();

    const BULK_SIZE: usize = 64;

    let start_time = Instant::now();

    for block_idx in 0..num_blocks {
        let block_start = output_start_offset + (block_idx * outputs_per_block);

        // Process outputs in bulk chunks
        let _outputs: Vec<_> = (0..outputs_per_block)
            .collect::<Vec<_>>()
            .par_chunks(BULK_SIZE)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .map(|&offset| {
                        let i = block_start + offset;
                        (
                            vecs.txoutindex_to_value
                                .read_at_unwrap(i, &txoutindex_to_value_reader),
                            vecs.txoutindex_to_outputtype
                                .read_at_unwrap(i, &txoutindex_to_outputtype_reader),
                            vecs.txoutindex_to_typeindex
                                .read_at_unwrap(i, &txoutindex_to_typeindex_reader),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Process inputs in bulk chunks
        let input_block_start = input_start_offset + (block_idx * inputs_per_block);

        let input_sum: u64 = (0..inputs_per_block)
            .collect::<Vec<_>>()
            .par_chunks(BULK_SIZE)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .filter_map(|&offset| {
                        let i = input_block_start + offset;
                        let outpoint = vecs
                            .txinindex_to_outpoint
                            .read_at_unwrap(i, &txinindex_to_outpoint_reader);

                        if outpoint.is_coinbase() {
                            return None;
                        }

                        let first_txoutindex = vecs.txindex_to_first_txoutindex.read_at_unwrap(
                            outpoint.txindex().to_usize(),
                            &txindex_to_first_txoutindex_reader,
                        );
                        let txoutindex = first_txoutindex.to_usize() + usize::from(outpoint.vout());
                        let value = vecs
                            .txoutindex_to_value
                            .read_at_unwrap(txoutindex, &txoutindex_to_value_reader);
                        Some(u64::from(value))
                    })
                    .collect::<Vec<_>>()
            })
            .sum();

        std::hint::black_box(input_sum);
    }

    start_time.elapsed()
}

fn calculate_stddev(times: &[Duration]) -> Duration {
    let avg = times.iter().sum::<Duration>().as_secs_f64() / times.len() as f64;
    let variance = times
        .iter()
        .map(|t| {
            let diff = t.as_secs_f64() - avg;
            diff * diff
        })
        .sum::<f64>()
        / times.len() as f64;
    Duration::from_secs_f64(variance.sqrt())
}
