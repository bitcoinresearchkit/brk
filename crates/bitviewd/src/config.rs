use std::{
    env, fs, io,
    net::IpAddr,
    path::{Path, PathBuf},
    process::exit,
};

use bitview::Config as RunnerConfig;
use bitview_server::{
    CdnCacheMode, DEFAULT_BIND, DEFAULT_MAX_UTXOS, DEFAULT_MAX_WEIGHT, ServerConfig, Website,
};
use brk_error::{Error, Result};
use brk_rpc::{Auth, Client};
use brk_types::Port;
use lexopt::{
    Arg::{Long, Short},
    Parser, ValueExt,
};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::paths::{default_bitview_dir, fix_user_path};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    bitviewdir: Option<String>,

    #[serde(default)]
    serverbind: Option<IpAddr>,

    #[serde(default)]
    serverport: Option<Port>,

    #[serde(default)]
    website: Option<Website>,

    #[serde(default)]
    cdn: Option<bool>,

    #[serde(default)]
    maxweight: Option<usize>,

    #[serde(default)]
    maxutxos: Option<usize>,

    #[serde(default)]
    bitcoindir: Option<String>,

    #[serde(default)]
    blocksdir: Option<String>,

    #[serde(default)]
    rpcconnect: Option<String>,

    #[serde(default)]
    rpcport: Option<u16>,

    #[serde(default)]
    rpccookiefile: Option<String>,

    #[serde(default)]
    rpcuser: Option<String>,

    #[serde(default)]
    rpcpassword: Option<String>,
}

impl Config {
    /// Load persisted settings without creating directories, validating Bitcoin
    /// paths, parsing arguments, or constructing an RPC client.
    pub fn load() -> io::Result<Self> {
        Self::read(&default_bitview_dir().join("config.toml"))
    }

    pub fn import() -> Result<RunnerConfig> {
        let config_args = Self::parse_args();

        let config_dir = default_bitview_dir();

        fs::create_dir_all(&config_dir)?;

        let config = Self::load()?.with_overrides(config_args);

        let data_path = config.bitviewdir();

        config.check();
        fs::create_dir_all(&data_path)?;

        config.runner()
    }

    fn with_overrides(self, overrides: Self) -> Self {
        Self {
            bitviewdir: overrides.bitviewdir.or(self.bitviewdir),
            serverbind: overrides.serverbind.or(self.serverbind),
            serverport: overrides.serverport.or(self.serverport),
            website: overrides.website.or(self.website),
            cdn: overrides.cdn.or(self.cdn),
            maxweight: overrides.maxweight.or(self.maxweight),
            maxutxos: overrides.maxutxos.or(self.maxutxos),
            bitcoindir: overrides.bitcoindir.or(self.bitcoindir),
            blocksdir: overrides.blocksdir.or(self.blocksdir),
            rpcconnect: overrides.rpcconnect.or(self.rpcconnect),
            rpcport: overrides.rpcport.or(self.rpcport),
            rpccookiefile: overrides.rpccookiefile.or(self.rpccookiefile),
            rpcuser: overrides.rpcuser.or(self.rpcuser),
            rpcpassword: overrides.rpcpassword.or(self.rpcpassword),
        }
    }

    fn parse_args() -> Self {
        let mut config = Self::default();
        let mut parser = Parser::from_env();
        let command = Self::command_name();

        while let Some(arg) = parser.next().unwrap() {
            match arg {
                Short('h') | Long("help") => {
                    Self::print_help(&command);
                    exit(0);
                }
                Short('V') | Long("version") => {
                    println!("{command} {}", env!("CARGO_PKG_VERSION"));
                    exit(0);
                }
                Long("bitviewdir") => {
                    config.bitviewdir = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("serverbind") => {
                    config.serverbind = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("serverport") => {
                    config.serverport = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("website") => config.website = Some(parser.value().unwrap().parse().unwrap()),
                Long("cdn") => config.cdn = Some(parser.value().unwrap().parse().unwrap()),
                Long("maxweight") => {
                    config.maxweight = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("maxutxos") => {
                    config.maxutxos = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("bitcoindir") => {
                    config.bitcoindir = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("blocksdir") => {
                    config.blocksdir = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("rpcconnect") => {
                    config.rpcconnect = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("rpcport") => config.rpcport = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpccookiefile") => {
                    config.rpccookiefile = Some(parser.value().unwrap().parse().unwrap())
                }
                Long("rpcuser") => config.rpcuser = Some(parser.value().unwrap().parse().unwrap()),
                Long("rpcpassword") => {
                    config.rpcpassword = Some(parser.value().unwrap().parse().unwrap())
                }
                _ => {
                    eprintln!("{}", arg.unexpected());
                    exit(1);
                }
            }
        }

        config
    }

    fn command_name() -> String {
        env::args_os()
            .next()
            .and_then(|path| {
                Path::new(&path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| "bitviewd".to_owned())
    }

    fn print_help(command: &str) {
        let v = env!("CARGO_PKG_VERSION");

        println!("{} {}", command.bold(), v.bright_black());
        println!("Self-hosted Bitcoin data platform built on BRK");
        println!();
        println!("{}", "USAGE:".bold());
        println!(
            "    {} {command} {}",
            "[ENV]".bright_black(),
            "[OPTIONS]".bright_black()
        );
        println!();
        println!("{}", "OPTIONS:".bold());
        println!("    -h, --help                Print help");
        println!("    -V, --version             Print version");
        println!();
        println!(
            "    --bitviewdir {}       Output directory {}",
            "<PATH>".bright_black(),
            "[~/.bitview]".bright_black()
        );
        println!(
            "    --serverbind {}       Server bind address {}",
            "<IP>".bright_black(),
            format!("[{DEFAULT_BIND}]").bright_black()
        );
        println!(
            "    --serverport {}       Server port {}",
            "<PORT>".bright_black(),
            format!("[{}]", Port::DEFAULT).bright_black()
        );
        println!(
            "    --website {}     Website {}",
            "<BOOL|PATH>".bright_black(),
            "[true]".bright_black()
        );
        println!(
            "    --cdn {}              Aggressive CDN cache, requires purge on deploy {}",
            "<BOOL>".bright_black(),
            "[false]".bright_black()
        );
        println!(
            "    --maxweight {}       Server cap on series response weight in bytes; rejects /api/{{series,metric}}/... over the limit {}",
            "<BYTES>".bright_black(),
            format!("[{}]", DEFAULT_MAX_WEIGHT).bright_black()
        );
        println!(
            "    --maxutxos {}        Server cap on UTXOs per address; /api/address/{{addr}}/utxo errors past the limit {}",
            "<COUNT>".bright_black(),
            format!("[{}]", DEFAULT_MAX_UTXOS).bright_black()
        );
        println!();
        println!(
            "    --bitcoindir {}       Bitcoin directory {}",
            "<PATH>".bright_black(),
            "[OS default]".bright_black()
        );
        println!(
            "    --blocksdir {}        Blocks directory {}",
            "<PATH>".bright_black(),
            "[<bitcoindir>/blocks]".bright_black()
        );
        println!();
        println!(
            "    --rpcconnect {}         RPC host {}",
            "<IP>".bright_black(),
            "[localhost]".bright_black()
        );
        println!(
            "    --rpcport {}          RPC port {}",
            "<PORT>".bright_black(),
            "[8332]".bright_black()
        );
        println!(
            "    --rpccookiefile {}    RPC cookie file {}",
            "<PATH>".bright_black(),
            "[<bitcoindir>/.cookie]".bright_black()
        );
        println!(
            "    --rpcuser {}      RPC username",
            "<USERNAME>".bright_black()
        );
        println!(
            "    --rpcpassword {}  RPC password",
            "<PASSWORD>".bright_black()
        );
        println!();
        println!("{}", "ENVIRONMENT:".bold());
        println!(
            "    LOG={}               Log level {}",
            "<LEVEL>".bright_black(),
            "[info]".bright_black()
        );
        println!(
            "    RUST_LOG={}          Full log filter",
            "<RULES>".bright_black()
        );
        println!();
        println!("{}", "CONFIG:".bold());
        println!(
            "    Edit {} to persist settings:",
            "~/.bitview/config.toml".bright_black()
        );
        println!("    {}", "bitviewdir = \"/path/to/data\"".bright_black());
        println!(
            "    {}",
            "bitcoindir = \"/path/to/.bitcoin\"".bright_black()
        );
    }

    fn check(&self) {
        if !self.bitcoindir().is_dir() {
            println!("{:?} isn't a valid directory", self.bitcoindir());
            println!("Please use the --bitcoindir parameter to set a valid path.");
            println!("Run the program with '-h' for help.");
            exit(1);
        }

        if !self.blocksdir().is_dir() {
            println!("{:?} isn't a valid directory", self.blocksdir());
            println!("Please use the --blocksdir parameter to set a valid path.");
            println!("Run the program with '-h' for help.");
            exit(1);
        }

        if self.rpc_auth().is_err() {
            println!(
                "Unsuccessful authentication with the RPC client.
First make sure that `bitcoind` is running. If it is then please either set --rpccookiefile or --rpcuser and --rpcpassword as the default values seemed to have failed.
Finally, you can run the program with '-h' for help."
            );
            exit(1);
        }
    }

    fn read(path: &Path) -> io::Result<Self> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Config::default()),
            Err(e) => {
                return Err(io::Error::new(
                    e.kind(),
                    format!("Cannot read {}: {e}", path.display()),
                ));
            }
        };
        from_str(&contents).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid config: {}", path.display()),
            )
        })
    }

    fn rpc(&self) -> Result<Client> {
        Client::new(
            &format!(
                "http://{}:{}",
                self.rpcconnect.as_deref().unwrap_or("localhost"),
                self.rpcport.unwrap_or(8332)
            ),
            self.rpc_auth()?,
        )
    }

    fn rpc_auth(&self) -> Result<Auth> {
        let cookie = self.path_cookiefile();

        if cookie.is_file() {
            Ok(Auth::CookieFile(cookie))
        } else if let (Some(user), Some(password)) = (&self.rpcuser, &self.rpcpassword) {
            Ok(Auth::UserPass(user.clone(), password.clone()))
        } else {
            Err(Error::AuthFailed)
        }
    }

    fn bitcoindir(&self) -> PathBuf {
        self.bitcoindir
            .as_ref()
            .map_or_else(Client::default_bitcoin_path, |s| fix_user_path(s.as_ref()))
    }

    fn blocksdir(&self) -> PathBuf {
        self.blocksdir.as_ref().map_or_else(
            || self.bitcoindir().join("blocks"),
            |blocksdir| fix_user_path(blocksdir.as_str()),
        )
    }

    fn bitviewdir(&self) -> PathBuf {
        self.bitviewdir
            .as_ref()
            .map_or_else(default_bitview_dir, |s| fix_user_path(s.as_ref()))
    }

    fn path_cookiefile(&self) -> PathBuf {
        self.rpccookiefile.as_ref().map_or_else(
            || self.bitcoindir().join(".cookie"),
            |p| fix_user_path(p.as_str()),
        )
    }

    fn website(&self) -> Website {
        self.website.clone().unwrap_or_default()
    }

    fn cdn_cache_mode(&self) -> CdnCacheMode {
        if self.cdn.unwrap_or(false) {
            CdnCacheMode::Aggressive
        } else {
            CdnCacheMode::Live
        }
    }

    fn max_weight(&self) -> usize {
        self.maxweight.unwrap_or(DEFAULT_MAX_WEIGHT)
    }

    fn max_utxos(&self) -> usize {
        self.maxutxos.unwrap_or(DEFAULT_MAX_UTXOS)
    }

    fn serverbind(&self) -> IpAddr {
        self.serverbind.unwrap_or(DEFAULT_BIND)
    }

    fn serverport(&self) -> Port {
        self.serverport.unwrap_or_default()
    }

    /// Resolve server settings without validating node paths or constructing RPC.
    pub fn server_config(&self) -> ServerConfig {
        ServerConfig {
            bind: self.serverbind(),
            port: self.serverport(),
            data_path: self.bitviewdir(),
            website: self.website(),
            cdn_cache_mode: self.cdn_cache_mode(),
            max_weight: self.max_weight(),
            max_utxos: self.max_utxos(),
        }
    }

    fn runner(&self) -> Result<RunnerConfig> {
        Ok(RunnerConfig {
            client: self.rpc()?,
            blocks_path: self.blocksdir(),
            server: self.server_config(),
        })
    }
}

#[cfg(test)]
#[path = "../tests/unit/config.rs"]
mod tests;
