use std::{
    fs,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use brk_computer::Computer;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::rpc::{self, Auth, Client, RpcApi};
use brk_server::tokio;
use clap::{Parser, ValueEnum};
use color_eyre::eyre::eyre;
use log::info;
use serde::{Deserialize, Serialize};

use crate::path_dot_brk;

pub fn run(config: RunConfig) -> color_eyre::Result<()> {
    let config = RunConfig::import(Some(config))?;

    let rpc = config.rpc()?;

    let exit = Exit::new();

    let parser = brk_parser::Parser::new(config.bitcoindir(), rpc);

    let mut indexer = Indexer::new(config.indexeddir())?;
    indexer.import_stores()?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(config.computeddir());
    computer.import_stores()?;
    computer.import_vecs()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let served_indexer = indexer.clone();
            let served_computer = computer.clone();

            let handle = if config.serve() {
                Some(tokio::spawn(async move {
                    brk_server::main(served_indexer, served_computer)
                        .await
                        .unwrap();
                }))
            } else {
                None
            };

            if config.process() {
                loop {
                    let block_count = rpc.get_block_count()?;

                    info!("{block_count} blocks found.");

                    let starting_indexes = indexer.index(&parser, rpc, &exit)?;

                    computer.compute(&mut indexer, starting_indexes, &exit)?;

                    if let Some(delay) = config.delay() {
                        sleep(Duration::from_secs(delay))
                    }

                    info!("Waiting for new blocks...");

                    while block_count == rpc.get_block_count()? {
                        sleep(Duration::from_secs(1))
                    }
                }
            }

            if let Some(handle) = handle {
                handle.await.unwrap();
            }
            Ok(())
        })
}

#[derive(Parser, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RunConfig {
    /// Bitcoin data directory path, saved
    #[arg(short, long, value_name = "PATH")]
    bitcoindir: Option<String>,

    /// Bitcoin Research Kit outputs directory path, saved
    #[arg(short, long, value_name = "PATH")]
    brkdir: Option<String>,

    /// Executed by the runner, default: all, saved
    #[arg(short, long)]
    mode: Option<Mode>,

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
}

impl RunConfig {
    pub fn import(config_args: Option<RunConfig>) -> color_eyre::Result<Self> {
        let path = path_dot_brk();

        let _ = fs::create_dir_all(&path);

        let path = path.join("config.toml");

        let mut config_saved = Self::read(&path);

        if let Some(mut config_args) = config_args {
            if let Some(bitcoindir) = config_args.bitcoindir.take() {
                config_saved.bitcoindir = Some(bitcoindir);
            }

            if let Some(brkdir) = config_args.brkdir.take() {
                config_saved.brkdir = Some(brkdir);
            }

            if let Some(mode) = config_args.mode.take() {
                config_saved.mode = Some(mode);
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

            if config_args != RunConfig::default() {
                dbg!(config_args);
                panic!("Didn't consume the full config")
            }
        }

        let config = config_saved;

        config.check();

        config.write(&path)?;

        // info!("Configuration {{");
        // info!("  bitcoindir: {:?}", config.bitcoindir);
        // info!("  brkdir: {:?}", config.brkdir);
        // info!("  mode: {:?}", config.mode);
        // info!("  rpcconnect: {:?}", config.rpcconnect);
        // info!("  rpcport: {:?}", config.rpcport);
        // info!("  rpccookiefile: {:?}", config.rpccookiefile);
        // info!("  rpcuser: {:?}", config.rpcuser);
        // info!("  rpcpassword: {:?}", config.rpcpassword);
        // info!("  delay: {:?}", config.delay);
        // info!("}}");

        Ok(config)
    }

    fn check(&self) {
        if self.bitcoindir.is_none() {
            println!(
                "You need to set the --bitcoindir parameter at least once to run the parser.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        } else if !self.bitcoindir().is_dir() {
            println!(
                "Given --bitcoindir parameter doesn't seem to be a valid directory path.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }

        if self.brkdir.is_none() {
            println!(
                "You need to set the --brkdir parameter at least once to run the parser.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        } else if !self.brkdir().is_dir() {
            println!(
                "Given --brkdir parameter doesn't seem to be a valid directory path.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }

        let path = self.bitcoindir();
        if !path.is_dir() {
            println!("Expect path '{:#?}' to be a directory.", path);
            std::process::exit(1);
        }

        if self.rpc_auth().is_err() {
            println!(
                "No way found to authenticate the RPC client, please either set --rpccookiefile or --rpcuser and --rpcpassword.\nRun the program with '-h' for help."
            );
            std::process::exit(1);
        }
    }

    fn read(path: &Path) -> Self {
        fs::read_to_string(path).map_or(RunConfig::default(), |contents| {
            toml::from_str(&contents).unwrap_or_default()
        })
    }

    fn write(&self, path: &Path) -> std::io::Result<()> {
        fs::write(path, toml::to_string(self).unwrap())
    }

    pub fn rpc(&self) -> color_eyre::Result<&'static Client> {
        Ok(Box::leak(Box::new(rpc::Client::new(
            &format!(
                "http://{}:{}",
                self.rpcconnect().unwrap_or(&"localhost".to_string()),
                self.rpcport().unwrap_or(8332)
            ),
            self.rpc_auth().unwrap(),
        )?)))
    }

    fn rpc_auth(&self) -> color_eyre::Result<Auth> {
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

    fn rpcconnect(&self) -> Option<&String> {
        self.rpcconnect.as_ref()
    }

    fn rpcport(&self) -> Option<u16> {
        self.rpcport
    }

    pub fn delay(&self) -> Option<u64> {
        self.delay
    }

    pub fn bitcoindir(&self) -> PathBuf {
        Self::fix_user_path(self.bitcoindir.as_ref().unwrap().as_ref())
    }

    pub fn brkdir(&self) -> PathBuf {
        Self::fix_user_path(self.brkdir.as_ref().unwrap().as_ref())
    }

    fn outputsdir(&self) -> PathBuf {
        self.brkdir().join("outputs")
    }

    pub fn indexeddir(&self) -> PathBuf {
        self.outputsdir().join("indexed")
    }

    pub fn computeddir(&self) -> PathBuf {
        self.outputsdir().join("computed")
    }

    pub fn process(&self) -> bool {
        self.mode
            .is_none_or(|m| m == Mode::All || m == Mode::Processor)
    }

    pub fn serve(&self) -> bool {
        self.mode
            .is_none_or(|m| m == Mode::All || m == Mode::Server)
    }

    fn path_cookiefile(&self) -> PathBuf {
        self.rpccookiefile.as_ref().map_or_else(
            || self.bitcoindir().join(".cookie"),
            |p| Self::fix_user_path(p.as_str()),
        )
    }

    fn fix_user_path(path: &str) -> PathBuf {
        let fix = move |pattern: &str| {
            if path.starts_with(pattern) {
                let path = &path
                    .replace(&format!("{pattern}/"), "")
                    .replace(pattern, "");

                let home = std::env::var("HOME").unwrap();

                Some(Path::new(&home).join(path))
            } else {
                None
            }
        };

        fix("~").unwrap_or_else(|| fix("$HOME").unwrap_or_else(|| PathBuf::from(&path)))
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    Parser,
    ValueEnum,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Mode {
    #[default]
    All,
    Processor,
    Server,
}
