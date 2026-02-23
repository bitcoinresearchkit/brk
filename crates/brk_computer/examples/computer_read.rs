use std::{env, path::Path, time::Instant};

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::{AnySerializableVec, AnyVec};

pub fn main() -> Result<()> {
    brk_logger::init(None)?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let computer = Computer::forced_import(&outputs_dir, &indexer)?;

    // Test emptyaddressdata (underlying BytesVec) - direct access
    let empty_data = &computer.distribution.addresses_data.empty;
    println!("emptyaddressdata (BytesVec) len: {}", empty_data.len());

    let start = Instant::now();
    let mut buf = Vec::new();
    empty_data.write_json(Some(empty_data.len() - 1), Some(empty_data.len()), &mut buf)?;
    println!(
        "emptyaddressdata last item JSON: {}",
        String::from_utf8_lossy(&buf)
    );
    println!("Time for BytesVec write_json: {:?}", start.elapsed());

    // Test emptyaddressindex (LazyVecFrom1 wrapper) - computed access
    let empty_index = &computer.distribution.emptyaddressindex;
    println!(
        "\nemptyaddressindex (LazyVecFrom1) len: {}",
        empty_index.len()
    );

    let start = Instant::now();
    let mut buf = Vec::new();
    empty_index.write_json(
        Some(empty_index.len() - 1),
        Some(empty_index.len()),
        &mut buf,
    )?;
    println!(
        "emptyaddressindex last item JSON: {}",
        String::from_utf8_lossy(&buf)
    );
    println!("Time for LazyVecFrom1 write_json: {:?}", start.elapsed());

    // Compare with funded versions
    let funded_data = &computer.distribution.addresses_data.funded;
    println!("\nfundedaddressdata (BytesVec) len: {}", funded_data.len());

    let start = Instant::now();
    let mut buf = Vec::new();
    funded_data.write_json(
        Some(funded_data.len() - 1),
        Some(funded_data.len()),
        &mut buf,
    )?;
    println!(
        "fundedaddressdata last item JSON: {}",
        String::from_utf8_lossy(&buf)
    );
    println!("Time for BytesVec write_json: {:?}", start.elapsed());

    Ok(())
}
