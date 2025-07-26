use std::{
    fs,
    path::{Path, PathBuf},
};

use bitcoincore_rpc::{self, Auth, Client};
use brk_core::{default_bitcoin_path, default_brk_path, default_on_error, dot_brk_path};
use brk_fetcher::Fetcher;
use brk_server::Website;
use brk_vecs::{Computation, Format};
use clap::Parser;
use clap_derive::Parser;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[command(version, about)]
pub struct Config {
    /// Bitcoin main directory path, defaults: ~/.bitcoin, ~/Library/Application\ Support/Bitcoin, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "PATH")]
    bitcoindir: Option<String>,

    /// Bitcoin blocks directory path, default: --bitcoindir/blocks, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "PATH")]
    blocksdir: Option<String>,

    /// Bitcoin Research Kit outputs directory path, default: ~/.brk, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "PATH")]
    brkdir: Option<String>,

    /// Computation of computed datasets, `lazy` computes data whenever requested without saving it, `eager` computes the data once and saves it to disk, default: `lazy`, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(short, long)]
    computation: Option<Computation>,

    /// Format of computed datasets, `compressed` to save disk space (experimental), `raw` to prioritize speed, default: `raw`, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(short, long)]
    format: Option<Format>,

    /// Activate fetching prices from exchanges APIs and the computation of all related datasets, default: true, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(short = 'F', long, value_name = "BOOL")]
    fetch: Option<bool>,

    /// Website served by the server (if active), default: default, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(short, long)]
    website: Option<Website>,

    /// Bitcoin RPC ip, default: localhost, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "IP")]
    rpcconnect: Option<String>,

    /// Bitcoin RPC port, default: 8332, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "PORT")]
    rpcport: Option<u16>,

    /// Bitcoin RPC cookie file, default: --bitcoindir/.cookie, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "PATH")]
    rpccookiefile: Option<String>,

    /// Bitcoin RPC username, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "USERNAME")]
    rpcuser: Option<String>,

    /// Bitcoin RPC password, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "PASSWORD")]
    rpcpassword: Option<String>,

    /// Delay between runs, default: 0, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "SECONDS")]
    delay: Option<u64>,

    /// Activate the Model Context Protocol (MCP) endpoint to give LLMs access to BRK (experimental), default: true, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "BOOL")]
    mcp: Option<bool>,

    /// DEV: Activate watching the selected website's folder for changes, default: false, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "BOOL")]
    watch: Option<bool>,

    /// DEV: Activate checking address hashes for collisions when indexing, default: false, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "BOOL")]
    check_collisions: Option<bool>,
}

impl Config {
    pub fn import() -> color_eyre::Result<Self> {
        let config_args = Some(Config::parse());

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

            if let Some(computation) = config_args.computation.take() {
                config_saved.computation = Some(computation);
            }

            if let Some(fetch) = config_args.fetch.take() {
                config_saved.fetch = Some(fetch);
            }

            if let Some(format) = config_args.format.take() {
                config_saved.format = Some(format);
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

            if let Some(mcp) = config_args.mcp.take() {
                config_saved.mcp = Some(mcp);
            }

            if let Some(watch) = config_args.watch.take() {
                config_saved.watch = Some(watch);
            }

            if config_args != Config::default() {
                dbg!(config_args);
                panic!("Didn't consume the full config")
            }
        }

        let config = config_saved;

        config.check();

        config.write(&path)?;

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
            |_| Config::default(),
            |contents| toml::from_str(&contents).unwrap_or_default(),
        )
    }

    fn write(&self, path: &Path) -> std::io::Result<()> {
        fs::write(path, toml::to_string(self).unwrap())
    }

    pub fn rpc(&self) -> color_eyre::Result<&'static Client> {
        Ok(Box::leak(Box::new(Client::new(
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

    pub fn harsdir(&self) -> PathBuf {
        self.brkdir().join("hars")
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
        self.website.unwrap_or(Website::Default)
    }

    pub fn fetch(&self) -> bool {
        self.fetch.is_none_or(|b| b)
    }

    pub fn fetcher(&self) -> Option<Fetcher> {
        self.fetch()
            .then(|| Fetcher::import(Some(self.harsdir().as_path())).unwrap())
    }

    pub fn computation(&self) -> Computation {
        self.computation.unwrap_or_default()
    }

    pub fn format(&self) -> Format {
        self.format.unwrap_or_default()
    }

    pub fn check_collisions(&self) -> bool {
        self.check_collisions.is_some_and(|b| b)
    }

    pub fn mcp(&self) -> bool {
        self.mcp.is_none_or(|b| b)
    }

    pub fn watch(&self) -> bool {
        self.watch.is_some_and(|b| b)
    }
}
