use std::{
    fs,
    path::{Path, PathBuf},
};

use bitcoincore_rpc::{self, Auth, Client};
use brk_fetcher::Fetcher;
use clap::Parser;
use clap_derive::Parser;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{default_bitcoin_path, default_brk_path, dot_brk_path, website::Website};

const DOWNLOADS: &str = "downloads";

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

    /// Activate fetching prices from BRK's API and the computation of all price related datasets, default: true, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(short = 'F', long, value_name = "BOOL")]
    fetch: Option<bool>,

    /// Activate fetching prices from exchanges APIs if `fetch` is also set to `true`, default: true, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(long, value_name = "BOOL")]
    exchanges: Option<bool>,

    /// Website served by the server, default: default, saved
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

    /// DEV: Activate checking address hashes for collisions when indexing, default: false, saved
    #[serde(default, deserialize_with = "default_on_error")]
    #[arg(skip)]
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

            if let Some(fetch) = config_args.fetch.take() {
                config_saved.fetch = Some(fetch);
            }

            if let Some(exchanges) = config_args.exchanges.take() {
                config_saved.exchanges = Some(exchanges);
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

            if let Some(check_collisions) = config_args.check_collisions.take() {
                config_saved.check_collisions = Some(check_collisions);
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
                "Unsuccessful authentication with the RPC client.
First make sure that `bitcoind` is running. If it is then please either set --rpccookiefile or --rpcuser and --rpcpassword as the default values seemed to have failed.
Finally, you can run the program with '-h' for help."
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

    pub fn downloads_dir(&self) -> PathBuf {
        dot_brk_path().join(DOWNLOADS)
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

    pub fn exchanges(&self) -> bool {
        self.exchanges.is_none_or(|b| b)
    }

    pub fn fetcher(&self) -> Option<Fetcher> {
        self.fetch()
            .then(|| Fetcher::import(self.exchanges(), Some(self.harsdir().as_path())).unwrap())
    }

    pub fn check_collisions(&self) -> bool {
        self.check_collisions.is_some_and(|b| b)
    }
}

fn default_on_error<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    match T::deserialize(deserializer) {
        Ok(v) => Ok(v),
        Err(_) => Ok(T::default()),
    }
}
