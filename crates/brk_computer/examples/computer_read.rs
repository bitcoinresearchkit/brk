use std::{env, path::Path, thread, time::Instant};

use brk_computer::Computer;
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use vecdb::{AnySerializableVec, AnyVec, Exit};

pub fn main() -> Result<()> {
    // Can't increase main thread's stack size, thus we need to use another thread
    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

fn run() -> Result<()> {
    brk_logger::init(None)?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let fetcher = Fetcher::import(None)?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let computer = Computer::forced_import(&outputs_dir, &indexer, Some(fetcher))?;

    // Test emptyaddressdata (underlying BytesVec) - direct access
    let empty_data = &computer.distribution.addresses_data.empty;
    println!("emptyaddressdata (BytesVec) len: {}", empty_data.len());

    let start = Instant::now();
    let mut buf = Vec::new();
    empty_data.write_json(Some(empty_data.len() - 1), Some(empty_data.len()), &mut buf)?;
    println!("emptyaddressdata last item JSON: {}", String::from_utf8_lossy(&buf));
    println!("Time for BytesVec write_json: {:?}", start.elapsed());

    // Test emptyaddressindex (LazyVecFrom1 wrapper) - computed access
    let empty_index = &computer.distribution.emptyaddressindex;
    println!("\nemptyaddressindex (LazyVecFrom1) len: {}", empty_index.len());

    let start = Instant::now();
    let mut buf = Vec::new();
    empty_index.write_json(Some(empty_index.len() - 1), Some(empty_index.len()), &mut buf)?;
    println!("emptyaddressindex last item JSON: {}", String::from_utf8_lossy(&buf));
    println!("Time for LazyVecFrom1 write_json: {:?}", start.elapsed());

    // Compare with loaded versions
    let loaded_data = &computer.distribution.addresses_data.loaded;
    println!("\nloadedaddressdata (BytesVec) len: {}", loaded_data.len());

    let start = Instant::now();
    let mut buf = Vec::new();
    loaded_data.write_json(Some(loaded_data.len() - 1), Some(loaded_data.len()), &mut buf)?;
    println!("loadedaddressdata last item JSON: {}", String::from_utf8_lossy(&buf));
    println!("Time for BytesVec write_json: {:?}", start.elapsed());

    Ok(())
}
