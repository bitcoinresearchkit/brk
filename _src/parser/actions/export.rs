use log::info;

use crate::{
    parser::{databases::Databases, datasets::Datasets, states::States},
    structs::{Config, Date, Exit, Height},
    utils::time,
};

pub struct ExportedData<'a> {
    pub config: &'a Config,
    pub databases: Option<&'a mut Databases>,
    pub datasets: &'a mut Datasets,
    pub date: Date,
    pub defragment: bool,
    pub exit: Exit,
    pub height: Height,
    pub states: Option<&'a States>,
}

pub fn export(
    ExportedData {
        config,
        databases,
        datasets,
        date,
        defragment,
        exit,
        height,
        states,
    }: ExportedData,
) -> color_eyre::Result<()> {
    if exit.active() {
        info!("Exit in progress, skipping export");
        return Ok(());
    }

    exit.block();

    let text = if defragment {
        "Exporting and defragmenting..."
    } else {
        "Exporting..."
    };
    info!("{text}");

    time("Finished export", || -> color_eyre::Result<()> {
        datasets.export(config, height)?;

        if let Some(databases) = databases {
            databases.export(height, date, defragment)?;
        }

        if let Some(states) = states {
            states.export(config)?;
        }

        Ok(())
    })?;

    exit.unblock();

    Ok(())
}
