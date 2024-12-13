use std::thread::{self};

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

    info!("Exporting...");
    if defragment {
        info!("Will also defragment databases, please be patient it might take a while")
    }

    time("Total save time", || -> color_eyre::Result<()> {
        time("Datasets saved", || datasets.export(config))?;

        thread::scope(|s| {
            if let Some(databases) = databases {
                s.spawn(|| {
                    time("Databases saved", || {
                        databases.export(height, date, defragment)
                    })
                });
            }

            if let Some(states) = states {
                s.spawn(|| time("States saved", || states.export(config)));
            }
        });

        Ok(())
    })?;

    exit.unblock();

    Ok(())
}
