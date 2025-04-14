use std::{
    fs,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use brk_computer::Computer;
use brk_core::{default_bitcoin_path, default_brk_path, dot_brk_path};
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::rpc::{self, Auth, Client, RpcApi};
use brk_server::{Server, Website, tokio};
use clap::{Parser, ValueEnum};
use color_eyre::eyre::eyre;
use log::info;
use serde::{Deserialize, Serialize};

pub fn run(config: RunConfig) -> color_eyre::Result<()> {
    let config = RunConfig::import(Some(config))?;

    let rpc = config.rpc()?;

    let exit = Exit::new();

    let parser = brk_parser::Parser::new(config.blocksdir(), rpc);

    let compressed = config.compressed();

    let mut indexer = Indexer::new(config.indexeddir(), compressed, config.check_collisions())?;
    indexer.import_stores()?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(config.computeddir(), config.fetcher(), compressed);
    computer.import_stores()?;
    computer.import_vecs()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let server = if config.serve() {
                let served_indexer = indexer.clone();
                let served_computer = computer.clone();

                let server = Server::new(served_indexer, served_computer, config.website())?;

                let opt = Some(tokio::spawn(async move {
                    server.serve().await.unwrap();
                }));

                sleep(Duration::from_secs(1));

                opt
            } else {
                None
            };

            if config.process() {
                let wait_for_synced_node = || -> color_eyre::Result<()> {
                    let is_synced = || -> color_eyre::Result<bool> {
                        let info = rpc.get_blockchain_info()?;
                        Ok(info.headers == info.blocks)
                    };

                    if !is_synced()? {
                        info!("Waiting for node to be synced...");
                        while !is_synced()? {
                            sleep(Duration::from_secs(1))
                        }
                    }

                    Ok(())
                };

                loop {
                    wait_for_synced_node()?;

                    let block_count = rpc.get_block_count()?;

                    info!("{} blocks found.", block_count + 1);

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

            if let Some(handle) = server {
                handle.await.unwrap();
            }

            Ok(())
        })
}

#[derive(Parser, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RunConfig {
    /// Bitcoin main directory path, defaults: ~/.bitcoin, ~/Library/Application\ Support/Bitcoin, saved
    #[arg(long, value_name = "PATH")]
    bitcoindir: Option<String>,

    /// Bitcoin blocks directory path, default: --bitcoindir/blocks, saved
    #[arg(long, value_name = "PATH")]
    blocksdir: Option<String>,

    /// Bitcoin Research Kit outputs directory path, default: ~/.brk, saved
    #[arg(long, value_name = "PATH")]
    brkdir: Option<String>,

    /// Executed by the runner, default: all, saved
    #[arg(short, long)]
    mode: Option<Mode>,

    /// Activate compression of datasets, set to true to save disk space or false if prioritize speed, default: true, saved
    #[arg(short, long, value_name = "BOOL")]
    compressed: Option<bool>,

    /// Activate fetching prices from exchanges APIs and the computation of all related datasets, default: true, saved
    #[arg(short, long, value_name = "BOOL")]
    fetch: Option<bool>,

    /// Website served by the server (if active), default: kibo.money, saved
    #[arg(short, long)]
    website: Option<Website>,

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

    /// DEV: Activate checking address hashes for collisions when indexing, default: false, saved
    #[arg(long, value_name = "BOOL")]
    check_collisions: Option<bool>,
}

impl RunConfig {
    pub fn import(config_args: Option<RunConfig>) -> color_eyre::Result<Self> {
        let path = dot_brk_path();

        let _ = fs::create_dir_all(&path);

        let path = path.join("config.toml");

        let mut config_saved = Self::read(&path);

        if let Some(mut config_args) = config_args {
            if let Some(bitcoindir) = config_args.bitcoindir.take() {
                config_saved.bitcoindir = Some(bitcoindir);
            }

            if let Some(blocksdir) = config_args.blocksdir.take() {
                config_saved.blocksdir = Some(blocksdir);
            }

            if let Some(brkdir) = config_args.brkdir.take() {
                config_saved.brkdir = Some(brkdir);
            }

            if let Some(mode) = config_args.mode.take() {
                config_saved.mode = Some(mode);
            }

            if let Some(fetch) = config_args.fetch.take() {
                config_saved.fetch = Some(fetch);
            }

            if let Some(compressed) = config_args.compressed.take() {
                config_saved.compressed = Some(compressed);
            }

            if let Some(website) = config_args.website.take() {
                config_saved.website = Some(website);
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

            if let Some(check_collisions) = config_args.check_collisions.take() {
                config_saved.check_collisions = Some(check_collisions);
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
        // info!("  website: {:?}", config.website);
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
        if !self.bitcoindir().is_dir() {
            println!("{:?} isn't a valid directory", self.bitcoindir());
            println!("Please use the --bitcoindir parameter to set a valid path.");
            println!("Run the program with '-h' for help.");
            std::process::exit(1);
        }

        if !self.blocksdir().is_dir() {
            println!("{:?} isn't a valid directory", self.blocksdir());
            println!("Please use the --blocksdir parameter to set a valid path.");
            println!("Run the program with '-h' for help.");
            std::process::exit(1);
        }

        if !self.brkdir().is_dir() {
            println!("{:?} isn't a valid directory", self.brkdir());
            println!("Please use the --brkdir parameter to set a valid path.");
            println!("Run the program with '-h' for help.");
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
        fs::read_to_string(path).map_or_else(
            |_| RunConfig::default(),
            |contents| toml::from_str(&contents).unwrap_or_default(),
        )
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
        self.bitcoindir
            .as_ref()
            .map_or_else(default_bitcoin_path, |s| Self::fix_user_path(s.as_ref()))
    }

    pub fn blocksdir(&self) -> PathBuf {
        self.blocksdir.as_ref().map_or_else(
            || self.bitcoindir().join("blocks"),
            |blocksdir| Self::fix_user_path(blocksdir.as_str()),
        )
    }

    pub fn brkdir(&self) -> PathBuf {
        self.brkdir
            .as_ref()
            .map_or_else(default_brk_path, |s| Self::fix_user_path(s.as_ref()))
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

    pub fn harsdir(&self) -> PathBuf {
        self.outputsdir().join("hars")
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

    pub fn website(&self) -> Website {
        self.website.unwrap_or(Website::KiboMoney)
    }

    pub fn fetch(&self) -> bool {
        self.fetch.is_none_or(|b| b)
    }

    pub fn fetcher(&self) -> Option<Fetcher> {
        self.fetch()
            .then(|| Fetcher::import(Some(self.harsdir().as_path())).unwrap())
    }

    pub fn compressed(&self) -> bool {
        self.compressed.is_none_or(|b| b)
    }

    pub fn check_collisions(&self) -> bool {
        self.check_collisions.is_some_and(|b| b)
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
