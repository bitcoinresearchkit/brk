#![doc = include_str!("../README.md")]

mod format;
mod hook;
mod rate_limit;

use std::{io, path::Path, time::Duration};

use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use format::Formatter;
use hook::{HookLayer, LOG_HOOK};
use rate_limit::{RateLimitedFile, is_log_file};

/// Days to keep log files before cleanup
const MAX_LOG_AGE_DAYS: u64 = 7;

/// Initialize the global tracing subscriber with a colorized console layer.
///
/// If `dir` is `Some`, also writes daily log files to that directory:
/// `YYYY-MM-DD.txt` for the combined log and `YYYY-MM-DD_<level>.txt` for each
/// tracing level. The directory is created if it does not exist, and any
/// `*.txt` file older than 7 days is pruned on startup.
pub fn init(dir: Option<&Path>) -> io::Result<()> {
    tracing_log::LogTracer::init().ok();

    #[cfg(debug_assertions)]
    const DEFAULT_LEVEL: &str = "debug";
    #[cfg(not(debug_assertions))]
    const DEFAULT_LEVEL: &str = "info";

    let level = std::env::var("LOG").unwrap_or_else(|_| DEFAULT_LEVEL.to_string());

    let directives = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "{level},bitcoin=off,corepc=off,tracing=off,aide=off,fjall=off,lsm_tree=off,tower_http=off"
        )
    });

    let filter: Targets = directives
        .parse()
        .unwrap_or_else(|_| Targets::new().with_default(tracing::Level::INFO));

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().event_format(Formatter::<true>))
        .with(HookLayer);

    if let Some(dir) = dir {
        let writer = RateLimitedFile::new(dir)?;

        cleanup_old_logs(dir);

        registry
            .with(
                fmt::layer()
                    .event_format(Formatter::<false>)
                    .with_writer(writer),
            )
            .init();
    } else {
        registry.init();
    }

    Ok(())
}

/// Register a hook that gets called for every log message.
pub fn register_hook<F>(hook: F) -> Result<(), &'static str>
where
    F: Fn(&str) + Send + Sync + 'static,
{
    LOG_HOOK
        .set(Box::new(hook))
        .map_err(|_| "Hook already registered")
}

fn cleanup_old_logs(dir: &Path) {
    let max_age = Duration::from_secs(MAX_LOG_AGE_DAYS * 24 * 60 * 60);
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !is_log_file(name) {
            continue;
        }

        if let Ok(meta) = path.metadata()
            && let Ok(modified) = meta.modified()
            && let Ok(age) = modified.elapsed()
            && age > max_age
        {
            let _ = std::fs::remove_file(&path);
        }
    }
}
