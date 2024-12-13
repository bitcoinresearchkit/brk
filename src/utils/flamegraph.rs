use std::{fs, path::PathBuf};

use chrono::Local;

use crate::{
    parser::{Databases, Datasets, States},
    structs::Height,
};

// use crate::{databases::Databases, datasets::AllDatasets, states::States, structs::Height};

pub fn generate_allocation_files(
    datasets: &Datasets,
    databases: &Databases,
    states: &States,
    last_height: Height,
) -> color_eyre::Result<()> {
    let mut flamegraph = allocative::FlameGraphBuilder::default();
    flamegraph.visit_root(datasets);
    flamegraph.visit_root(databases);
    flamegraph.visit_root(states);
    let output = flamegraph.finish();

    let folder = format!(
        "at-{}-result-of-{}",
        Local::now().format("%Y-%m-%d_%Hh%Mm%Ss"),
        last_height
    );

    let path = PathBuf::from(&format!("./target/flamegraph/{folder}"));
    fs::create_dir_all(&path)?;

    // fs::write(path.join("flamegraph.src"), &output.flamegraph())?;

    let mut fg_svg = Vec::new();
    inferno::flamegraph::from_reader(
        &mut inferno::flamegraph::Options::default(),
        output.flamegraph().write().as_bytes(),
        &mut fg_svg,
    )?;

    fs::write(path.join("flamegraph.svg"), &fg_svg)?;

    fs::write(path.join("warnings.txt"), output.warnings())?;

    Ok(())
}
