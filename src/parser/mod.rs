use std::{thread::sleep, time::Duration};

use biter::bitcoincore_rpc::{Client, RpcApi};

mod actions;
mod databases;
mod datasets;
mod price;
mod states;

pub use actions::*;
pub use databases::*;
pub use datasets::*;
use log::info;
pub use states::*;

use crate::structs::{Config, Exit};

pub fn main(
    config: &Config,
    rpc: &Client,
    exit: &Exit,
    mut databases: Databases,
    mut datasets: Datasets,
) -> color_eyre::Result<()> {
    loop {
        let block_count = rpc.get_blockchain_info().unwrap().blocks as usize;

        info!("{block_count} blocks found.");

        iter_blocks(
            config,
            rpc,
            block_count,
            exit.clone(),
            &mut databases,
            &mut datasets,
        )?;

        if let Some(delay) = config.delay() {
            sleep(Duration::from_secs(delay))
        }

        info!("Waiting for a new block...");

        while block_count == rpc.get_blockchain_info().unwrap().blocks as usize {
            sleep(Duration::from_secs(1))
        }
    }

    // Ok(())
}
