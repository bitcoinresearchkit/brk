use std::{
    fs::{self},
    mem,
    path::{Path, PathBuf},
};

use brk_parser::bitcoincore_rpc::Auth;
use clap::Parser;
use color_eyre::eyre::eyre;
use log::info;
use serde::{Deserialize, Serialize};

use crate::io::JSON_EXTENSION;

use super::MapPath;

#[derive(Parser, Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Bitcoin data directory path, saved
    #[arg(long, value_name = "PATH")]
    bitcoindir: Option<String>,

    /// Kibo output directory path, saved
    #[arg(long, value_name = "PATH")]
    kibodir: Option<String>,

    /// Bitcoin RPC ip, default: localhost, saved
    #[arg(long, value_name = "IP")]
    rpcconnect: Option<String>,

    /// Bitcoin RPC port, default: 8332, saved
    #[arg(long, value_name = "PORT")]
    rpcport: Option<u16>,

    /// Bitcoin RPC cookie file, default: --bitcoindir/.cookie, saved
    #[arg(long, value_name = "PATH")]
    rpccookiefile: Option<String>,

    /// Bitcoin RPC username, saved
    #[arg(long, value_name = "USERNAME")]
    rpcuser: Option<String>,

    /// Bitcoin RPC password, saved
    #[arg(long, value_name = "PASSWORD")]
    rpcpassword: Option<String>,

    /// Delay between runs, default: 0, saved
    #[arg(long, value_name = "SECONDS")]
    delay: Option<u64>,

    /// Disable the parser, not saved
    #[serde(default)]
    #[arg(long, default_value_t = false)]
    no_parser: bool,

    /// Disable the server, not saved
    #[serde(default)]
    #[arg(long, default_value_t = false)]
    no_server: bool,

    /// Run without saving, not saved
    #[serde(default)]
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Record ram usage, not saved
    #[serde(default)]
    #[arg(long, default_value_t = false)]
    record_ram_usage: bool,

    /// Recompute all computed datasets, not saved
    #[serde(default)]
    #[arg(long, default_value_t = false)]
    recompute_computed: bool,
}

impl Config {
    pub const DATASET_DIR_NAME: &str = "datasets";
    pub const DATABASES_DIR_NAME: &str = "databases";

    pub fn import() -> color_eyre::Result<Self> {
        let path = Self::path_dot_kibo();
        let _ = fs::create_dir_all(&path);

        let path = path.join("config.toml");

        let mut config_saved = Self::read(&path);

        let mut config_args = Config::parse();

        if let Some(bitcoindir) = config_args.bitcoindir.take() {
            config_saved.bitcoindir = Some(bitcoindir);
        }

        if let Some(kibodir) = config_args.kibodir.take() {
            config_saved.kibodir = Some(kibodir);
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

        config.write(&path)?;

        config.no_parser = mem::take(&mut config_args.no_parser);
        config.no_server = mem::take(&mut config_args.no_server);
        config.dry_run = mem::take(&mut config_args.dry_run);
        config.record_ram_usage = mem::take(&mut config_args.record_ram_usage);
        config.recompute_computed = mem::take(&mut config_args.recompute_computed);

        info!("Configuration {{");
        info!("  bitcoindir: {:?}", config.bitcoindir);
        info!("  kibodir: {:?}", config.kibodir);
        info!("  rpcconnect: {:?}", config.rpcconnect);
        info!("  rpcport: {:?}", config.rpcport);
        info!("  rpccookiefile: {:?}", config.rpccookiefile);
        info!("  rpcuser: {:?}", config.rpcuser);
        info!("  rpcpassword: {:?}", config.rpcpassword);
        info!("  delay: {:?}", config.delay);
        // info!("  max_ram: {:?}", config.max_ram);
        info!("  parser: {:?}", config.parser());
        info!("  server: {:?}", config.server());
        info!("  dry_run: {:?}", config.dry_run());
        info!("  record_ram_usage: {:?}", config.record_ram_usage());
        info!("  recompute_computed: {:?}", config.recompute_computed());
        info!("}}");

        if config_args != Config::default() {
            dbg!(config_args);
            panic!("Didn't consume the full config")
        }

        Ok(config)
    }

    fn check(&self) {
        if self.bitcoindir.is_none() {
            println!(
                "You need to set the --bitcoindir parameter at least once to run the parser.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        } else if !self.path_bitcoindir().is_dir() {
            println!(
                "Given --bitcoindir parameter doesn't seem to be a valid directory path.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }

        if self.kibodir.is_none() {
            println!(
                "You need to set the --kibodir parameter at least once to run the parser.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        } else if !self.path_kibodir().is_dir() {
            println!(
                "Given --kibodir parameter doesn't seem to be a valid directory path.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }

        let path = self.path_bitcoindir();
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

    fn read(path: &Path) -> Self {
        fs::read_to_string(path).map_or(Config::default(), |contents| {
            toml::from_str(&contents).unwrap_or_default()
        })
    }

    fn write(&self, path: &Path) -> std::io::Result<()> {
        fs::write(path, toml::to_string(self).unwrap())
    }

    pub fn to_rpc_auth(&self) -> color_eyre::Result<Auth> {
        let cookie = self.path_cookiefile();

        if cookie.is_file() {
            Ok(Auth::CookieFile(cookie))
        } else if self.rpcuser.is_some() && self.rpcpassword.is_some() {
            Ok(Auth::UserPass(
                self.rpcuser.clone().unwrap(),
                self.rpcpassword.clone().unwrap(),
            ))
        } else {
            Err(eyre!("Failed to find correct auth"))
        }
    }

    pub fn rpcconnect(&self) -> Option<&String> {
        self.rpcconnect.as_ref()
    }

    pub fn rpcport(&self) -> Option<u16> {
        self.rpcport
    }

    pub fn delay(&self) -> Option<u64> {
        self.delay
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    pub fn record_ram_usage(&self) -> bool {
        self.record_ram_usage
    }

    pub fn recompute_computed(&self) -> bool {
        self.recompute_computed
    }

    pub fn path_bitcoindir(&self) -> PathBuf {
        Self::fix_user_path(self.bitcoindir.as_ref().unwrap().as_ref())
    }

    pub fn path_kibodir(&self) -> PathBuf {
        Self::fix_user_path(self.kibodir.as_ref().unwrap().as_ref())
    }

    fn path_cookiefile(&self) -> PathBuf {
        self.rpccookiefile.as_ref().map_or_else(
            || self.path_bitcoindir().join(".cookie"),
            |p| Self::fix_user_path(p.as_str()),
        )
    }

    fn fix_user_path(path: &str) -> PathBuf {
        let fix = move |pattern: &str| {
            if path.starts_with(pattern) {
                let path = &path.replace(&format!("{pattern}/"), "").replace(pattern, "");

                let home = std::env::var("HOME").unwrap();

                Some(Path::new(&home).join(path))
            } else {
                None
            }
        };

        fix("~").unwrap_or_else(|| fix("$HOME").unwrap_or_else(|| PathBuf::from(&path)))
    }

    pub fn path_datasets(&self) -> MapPath {
        MapPath::from(self.path_kibodir().join(Self::DATASET_DIR_NAME))
    }

    pub fn path_datasets_last_values(&self) -> MapPath {
        self.path_datasets().join(&format!("last.{JSON_EXTENSION}"))
    }

    pub fn path_price(&self) -> MapPath {
        MapPath::from(self.path_kibodir().join("price"))
    }

    pub fn path_databases(&self) -> PathBuf {
        self.path_kibodir().join(Self::DATABASES_DIR_NAME)
    }

    pub fn path_states(&self) -> PathBuf {
        self.path_kibodir().join("states")
    }

    pub fn path_inputs(&self) -> PathBuf {
        self.path_kibodir().join("inputs")
    }

    fn path_dot_kibo() -> PathBuf {
        let home = std::env::var("HOME").unwrap();
        Path::new(&home).join(".kibo")
    }

    pub fn path_log() -> PathBuf {
        Self::path_dot_kibo().join("log")
    }

    pub fn parser(&self) -> bool {
        !self.no_parser
    }

    pub fn server(&self) -> bool {
        !self.no_server
    }
}
