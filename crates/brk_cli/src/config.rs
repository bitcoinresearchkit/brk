use std::{
    fs,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_fetcher::Fetcher;
use brk_rpc::{Auth, Client};
use brk_types::Port;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{default_brk_path, dot_brk_path, fix_user_path, website::WebsiteArg};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Config {
    #[serde(default, deserialize_with = "default_on_error")]
    brkdir: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    brkport: Option<Port>,

    #[serde(default, deserialize_with = "default_on_error")]
    website: Option<WebsiteArg>,

    #[serde(default, deserialize_with = "default_on_error")]
    fetch: Option<bool>,

    #[serde(default, deserialize_with = "default_on_error")]
    bitcoindir: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    blocksdir: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    rpcconnect: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    rpcport: Option<u16>,

    #[serde(default, deserialize_with = "default_on_error")]
    rpccookiefile: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    rpcuser: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    rpcpassword: Option<String>,

    #[serde(default, deserialize_with = "default_on_error")]
    check_collisions: Option<bool>,
}

impl Config {
    pub fn import() -> Result<Self> {
        let config_args = Self::parse_args();

        let path = dot_brk_path();

        let _ = fs::create_dir_all(&path);

        let path = path.join("config.toml");

        let mut config = Self::read(&path);

        if let Some(v) = config_args.brkdir {
            config.brkdir = Some(v);
        }
        if let Some(v) = config_args.brkport {
            config.brkport = Some(v);
        }
        if let Some(v) = config_args.website {
            config.website = Some(v);
        }
        if let Some(v) = config_args.fetch {
            config.fetch = Some(v);
        }
        if let Some(v) = config_args.bitcoindir {
            config.bitcoindir = Some(v);
        }
        if let Some(v) = config_args.blocksdir {
            config.blocksdir = Some(v);
        }
        if let Some(v) = config_args.rpcconnect {
            config.rpcconnect = Some(v);
        }
        if let Some(v) = config_args.rpcport {
            config.rpcport = Some(v);
        }
        if let Some(v) = config_args.rpccookiefile {
            config.rpccookiefile = Some(v);
        }
        if let Some(v) = config_args.rpcuser {
            config.rpcuser = Some(v);
        }
        if let Some(v) = config_args.rpcpassword {
            config.rpcpassword = Some(v);
        }
        if let Some(v) = config_args.check_collisions {
            config.check_collisions = Some(v);
        }

        config.check();

        config.write(&path)?;

        Ok(config)
    }

    fn parse_args() -> Self {
        use lexopt::prelude::*;

        let mut config = Self::default();
        let mut parser = lexopt::Parser::from_env();

        while let Some(arg) = parser.next().unwrap() {
            match arg {
                Short('h') | Long("help") => {
                    Self::print_help();
                    std::process::exit(0);
                }
                Short('V') | Long("version") => {
                    println!("brk {}", env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                Long("brkdir") => config.brkdir = Some(parser.value().unwrap().parse().unwrap()),
                Long("brkport") => config.brkport = Some(parser.value().unwrap().parse().unwrap()),
                Long("website") => config.website = Some(parser.value().unwrap().parse().unwrap()),
                Long("fetch") => config.fetch = Some(parser.value().unwrap().parse().unwrap()),
                Long("bitcoindir") => config.bitcoindir = Some(parser.value().unwrap().parse().unwrap()),
                Long("blocksdir") => config.blocksdir = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpcconnect") => config.rpcconnect = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpcport") => config.rpcport = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpccookiefile") => config.rpccookiefile = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpcuser") => config.rpcuser = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpcpassword") => config.rpcpassword = Some(parser.value().unwrap().parse().unwrap()),
                Long("check-collisions") => config.check_collisions = Some(parser.value().unwrap().parse().unwrap()),
                _ => {
                    eprintln!("{}", arg.unexpected());
                    std::process::exit(1);
                }
            }
        }

        config
    }

    fn print_help() {
        println!(
            "brk {}
Bitcoin Research Kit

USAGE:
    brk [OPTIONS]

OPTIONS:
    -h, --help                   Print help
    -V, --version                Print version

    --brkdir <PATH>              Output directory [~/.brk]
    --brkport <PORT>             Server port [3110]
    --website <BOOL|PATH>        Website: true, false, or path [true]
    --fetch <BOOL>               Fetch prices [true]

    --bitcoindir <PATH>          Bitcoin directory [~/.bitcoin, ~/Library/Application Support/Bitcoin]
    --blocksdir <PATH>           Blocks directory [<bitcoindir>/blocks]

    --rpcconnect <IP>            RPC host [localhost]
    --rpcport <PORT>             RPC port [8332]
    --rpccookiefile <PATH>       RPC cookie file [<bitcoindir>/.cookie]
    --rpcuser <USERNAME>         RPC username
    --rpcpassword <PASSWORD>     RPC password",
            env!("CARGO_PKG_VERSION")
        );
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

    pub fn rpc(&self) -> Result<Client> {
        Client::new(
            &format!(
                "http://{}:{}",
                self.rpcconnect().unwrap_or(&"localhost".to_string()),
                self.rpcport().unwrap_or(8332)
            ),
            self.rpc_auth()?,
        )
    }

    fn rpc_auth(&self) -> Result<Auth> {
        let cookie = self.path_cookiefile();

        if cookie.is_file() {
            Ok(Auth::CookieFile(cookie))
        } else if self.rpcuser.is_some() && self.rpcpassword.is_some() {
            Ok(Auth::UserPass(
                self.rpcuser.clone().unwrap(),
                self.rpcpassword.clone().unwrap(),
            ))
        } else {
            Err(Error::AuthFailed)
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
            .map_or_else(Client::default_bitcoin_path, |s| fix_user_path(s.as_ref()))
    }

    pub fn blocksdir(&self) -> PathBuf {
        self.blocksdir.as_ref().map_or_else(
            || self.bitcoindir().join("blocks"),
            |blocksdir| fix_user_path(blocksdir.as_str()),
        )
    }

    pub fn brkdir(&self) -> PathBuf {
        self.brkdir
            .as_ref()
            .map_or_else(default_brk_path, |s| fix_user_path(s.as_ref()))
    }

    pub fn harsdir(&self) -> PathBuf {
        self.brkdir().join("hars")
    }

    fn path_cookiefile(&self) -> PathBuf {
        self.rpccookiefile.as_ref().map_or_else(
            || self.bitcoindir().join(".cookie"),
            |p| fix_user_path(p.as_str()),
        )
    }

    pub fn website(&self) -> WebsiteArg {
        self.website.clone().unwrap_or_default()
    }

    pub fn brkport(&self) -> Option<Port> {
        self.brkport
    }

    pub fn fetch(&self) -> bool {
        self.fetch.is_none_or(|b| b)
    }

    pub fn fetcher(&self) -> Option<Fetcher> {
        self.fetch()
            .then(|| Fetcher::import(Some(self.harsdir().as_path())).unwrap())
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
