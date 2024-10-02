use std::thread::{self};

use crate::{
    databases::Databases,
    datasets::AllDatasets,
    states::States,
    structs::{Date, Height},
    utils::{log, time},
    Exit,
};

pub struct ExportedData<'a> {
    pub databases: Option<&'a mut Databases>,
    pub datasets: &'a mut AllDatasets,
    pub date: Date,
    pub height: Height,
    pub states: Option<&'a States>,
    pub exit: Exit,
}

pub fn export(
    ExportedData {
        databases,
        datasets,
        states,
        height,
        date,
        exit,
    }: ExportedData,
) -> color_eyre::Result<()> {
    if exit.active() {
        log("Exit in progress, skipping export");
        return Ok(());
    }

    exit.block();

    log("Exporting...");

    time("Total save time", || -> color_eyre::Result<()> {
        time("Datasets saved", || datasets.export())?;

        thread::scope(|s| {
            if let Some(databases) = databases {
                s.spawn(|| time("Databases saved", || databases.export(height, date)));
            }

            if let Some(states) = states {
                s.spawn(|| time("States saved", || states.export()));
            }
        });

        Ok(())
    })?;

    exit.unblock();

    Ok(())
}
