use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use biter::bitcoincore_rpc::Auth;
use clap::Parser;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

use crate::log;

#[derive(Parser, Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Bitcoin data directory path, saved
    #[arg(long, value_name = "DIR")]
    pub datadir: Option<String>,

    /// Bitcoin RPC ip, default: localhost, saved
    #[arg(long, value_name = "IP")]
    pub rpcconnect: Option<String>,

    /// Bitcoin RPC port, default: 8332, saved
    #[arg(long, value_name = "PORT")]
    pub rpcport: Option<u16>,

    /// Bitcoin RPC cookie file, saved
    #[arg(long, value_name = "PATH")]
    pub rpccookiefile: Option<String>,

    /// Bitcoin RPC username, saved
    #[arg(long, value_name = "USERNAME")]
    pub rpcuser: Option<String>,

    /// Bitcoin RPC password, saved
    #[arg(long, value_name = "PASSWORD")]
    pub rpcpassword: Option<String>,

    /// Delay between runs, default: 0, saved
    #[arg(long, value_name = "SECONDS")]
    pub delay: Option<u64>,

    // Maximum ram you want the program to use in GB, default: 50% of total, not saved
    // #[arg(long, value_name = "GB")]
    // pub max_ram: Option<f64>,
    /// Start a dry run, default: false, not saved
    #[arg(long, value_name = "BOOL")]
    dry_run: Option<bool>,

    /// Record ram usage, default: false, not saved
    #[arg(long, value_name = "BOOL")]
    record_ram_usage: Option<bool>,

    /// Recompute all computed datasets, default: false, not saved
    #[arg(long, value_name = "BOOL")]
    recompute_computed: Option<bool>,

    /// Start the program by defragmenting all databases to reduce their footprint, default: false, not saved
    #[arg(long, value_name = "BOOL")]
    first_defragment: Option<bool>,
}

impl Config {
    const PATH: &'static str = "./config.toml";

    pub fn import() -> color_eyre::Result<Self> {
        let mut config_saved = fs::read_to_string(Self::PATH)
            .map_or(Config::default(), |contents| {
                toml::from_str(&contents).unwrap_or_default()
            });

        let mut config_args = Config::parse();

        if let Some(datadir) = config_args.datadir.take() {
            config_saved.datadir = Some(datadir);
        }

        if let Some(rpcconnect) = config_args.rpcconnect.take() {
            config_saved.rpcconnect = Some(rpcconnect);
        }

        if let Some(rpcport) = config_args.rpcport.take() {
            config_saved.rpcport = Some(rpcport);
        }

        if let Some(rpccookiefile) = config_args.rpccookiefile.take() {
            config_saved.rpccookiefile = Some(rpccookiefile);
        }

        if let Some(rpcuser) = config_args.rpcuser.take() {
            config_saved.rpcuser = Some(rpcuser);
        }

        if let Some(rpcpassword) = config_args.rpcpassword.take() {
            config_saved.rpcpassword = Some(rpcpassword);
        }

        if let Some(delay) = config_args.delay.take() {
            config_saved.delay = Some(delay);
        }

        // if let Some(max_ram) = config_args.max_ram.take() {
        //     config_saved.max_ram = Some(max_ram);
        // }

        // Done importing

        let mut config = config_saved;

        config.check();

        config.write()?;

        config.dry_run = config_args.dry_run.take();
        config.record_ram_usage = config_args.record_ram_usage.take();
        config.recompute_computed = config_args.recompute_computed.take();
        config.first_defragment = config_args.first_defragment.take();

        log("---");
        log("Configuration:");
        log(&format!("datadir: {:?}", config.datadir));
        log(&format!("rpcconnect: {:?}", config.rpcconnect));
        log(&format!("rpcport: {:?}", config.rpcport));
        log(&format!("rpccookiefile: {:?}", config.rpccookiefile));
        log(&format!("rpcuser: {:?}", config.rpcuser));
        log(&format!("rpcpassword: {:?}", config.rpcpassword));
        log(&format!("delay: {:?}", config.delay));
        // log(&format!("max_ram: {:?}", config.max_ram));
        log(&format!("dry_run: {:?}", config.dry_run));
        log(&format!("record_ram_usage: {:?}", config.record_ram_usage));
        log(&format!(
            "recompute_computed: {:?}",
            config.recompute_computed
        ));
        log(&format!("first_defragment: {:?}", config.first_defragment));
        log("---");

        if config_args != Config::default() {
            dbg!(config_args);
            panic!("Didn't consume the full config")
        }

        Ok(config)
    }

    fn check(&self) {
        if self.datadir.is_none() {
            println!(
                "You need to set the --datadir parameter at least once to run the parser.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }

        let path = Path::new(self.datadir.as_ref().unwrap());
        if !path.is_dir() {
            println!("Expect path '{:#?}' to be a directory.", path);
            std::process::exit(1);
        }

        if self.to_rpc_auth().is_err() {
            println!(
                "No way found to authenticate the RPC client, please either set --rpccookiefile or --rpcuser and --rpcpassword.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }
    }

    fn write(&self) -> std::io::Result<()> {
        fs::write(Self::PATH, toml::to_string(self).unwrap())
    }

    pub fn to_rpc_auth(&self) -> color_eyre::Result<Auth> {
        let cookie = Path::new(self.datadir.as_ref().unwrap()).join(".cookie");

        if cookie.is_file() {
            Ok(Auth::CookieFile(cookie))
        } else if self
            .rpccookiefile
            .as_ref()
            .is_some_and(|cookie| Path::new(cookie).is_file())
        {
            Ok(Auth::CookieFile(PathBuf::from(
                self.rpccookiefile.as_ref().unwrap(),
            )))
        } else if self.rpcuser.is_some() && self.rpcpassword.is_some() {
            Ok(Auth::UserPass(
                self.rpcuser.clone().unwrap(),
                self.rpcpassword.clone().unwrap(),
            ))
        } else {
            Err(eyre!("Failed to find correct auth"))
        }
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run.is_some_and(|b| b)
    }

    pub fn record_ram_usage(&self) -> bool {
        self.record_ram_usage.is_some_and(|b| b)
    }

    pub fn recompute_computed(&self) -> bool {
        self.recompute_computed.is_some_and(|b| b)
    }

    pub fn first_defragment(&self) -> bool {
        self.first_defragment.is_some_and(|b| b)
    }

    pub fn disable_defragment(&mut self) {
        self.first_defragment.take();
    }
}
