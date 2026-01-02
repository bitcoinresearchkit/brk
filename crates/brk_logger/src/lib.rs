#![doc = include_str!("../README.md")]

use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::{self, BufWriter, Write},
    path::Path,
    sync::OnceLock,
};

use env_logger::{Builder, Env};
use jiff::{Timestamp, tz};
pub use owo_colors::OwoColorize;
use parking_lot::Mutex;

// Type alias for the hook function
type LogHook = Box<dyn Fn(&str) + Send + Sync>;

static LOG_HOOK: OnceLock<LogHook> = OnceLock::new();
static LOG_FILE: OnceLock<Mutex<BufWriter<fs::File>>> = OnceLock::new();

#[inline]
pub fn init(path: Option<&Path>) -> io::Result<()> {
    if let Some(path) = path {
        let _ = fs::remove_file(path);
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        LOG_FILE.set(Mutex::new(BufWriter::new(file))).ok();
    }

    #[cfg(debug_assertions)]
    let default_level = "debug";
    #[cfg(not(debug_assertions))]
    let default_level = "info";

    let filter = format!(
        "{default_level},bitcoin=off,bitcoincore-rpc=off,fjall=off,brk_fjall=off,lsm_tree=off,brk_rolldown=off,rolldown=off,rmcp=off,brk_rmcp=off,tracing=off,aide=off,rustls=off,notify=off,oxc_resolver=off,tower_http=off"
    );

    Builder::from_env(Env::default().default_filter_or(filter))
        .format(move |buf, record| {
            let date_time = Timestamp::now()
                .to_zoned(tz::TimeZone::system())
                .strftime("%Y-%m-%d %H:%M:%S")
                .to_string();
            let level = record.level().as_str().to_lowercase();
            let level = format!("{level:5}");
            let target = record.target();
            let dash = "-";
            let args = record.args();

            if let Some(hook) = LOG_HOOK.get() {
                hook(&args.to_string());
            }

            if let Some(file) = LOG_FILE.get() {
                let _ = write(&mut *file.lock(), &date_time, target, &level, dash, args);
            }

            let colored_date_time = date_time.bright_black();
            let colored_level = match level.chars().next().unwrap() {
                'e' => level.red().to_string(),
                'w' => level.yellow().to_string(),
                'i' => level.green().to_string(),
                'd' => level.blue().to_string(),
                't' => level.cyan().to_string(),
                _ => panic!(),
            };
            let colored_dash = dash.bright_black();

            write(
                buf,
                colored_date_time,
                target,
                colored_level,
                colored_dash,
                args,
            )
        })
        .init();

    Ok(())
}

/// Register a hook that gets called for every log message
/// Can only be called once
pub fn register_hook<F>(hook: F) -> Result<(), &'static str>
where
    F: Fn(&str) + Send + Sync + 'static,
{
    LOG_HOOK
        .set(Box::new(hook))
        .map_err(|_| "Hook already registered")
}

fn write(
    mut buf: impl Write,
    date_time: impl Display,
    _target: impl Display,
    level: impl Display,
    dash: impl Display,
    args: impl Display,
) -> Result<(), std::io::Error> {
    writeln!(buf, "{date_time} {dash} {level} {args}")
    // Don't remove, used to know the target of unwanted logs
    // writeln!(
    //     buf,
    //     "{} {} {} {}  {}",
    //     date_time, _target, level, dash, args
    // )
}
