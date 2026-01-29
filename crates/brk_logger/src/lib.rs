#![doc = include_str!("../README.md")]

mod format;
mod hook;
mod rate_limit;

use std::{io, path::Path, time::Duration};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use format::Formatter;
use hook::{HookLayer, LOG_HOOK};
use rate_limit::RateLimitedFile;

/// Days to keep log files before cleanup
const MAX_LOG_AGE_DAYS: u64 = 7;

pub fn init(path: Option<&Path>) -> io::Result<()> {
    tracing_log::LogTracer::init().ok();

    #[cfg(debug_assertions)]
    const DEFAULT_LEVEL: &str = "debug";
    #[cfg(not(debug_assertions))]
    const DEFAULT_LEVEL: &str = "info";

    let level = std::env::var("LOG").unwrap_or_else(|_| DEFAULT_LEVEL.to_string());

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "{level},bitcoin=off,bitcoincore-rpc=off,fjall=off,brk_fjall=off,lsm_tree=off,brk_rolldown=off,rolldown=off,tracing=off,aide=off,rustls=off,notify=off,oxc_resolver=off,tower_http=off"
        ))
    });

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().event_format(Formatter::<true>))
        .with(HookLayer);

    if let Some(path) = path {
        let dir = path.parent().unwrap_or(Path::new("."));
        let prefix = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("app.log");

        cleanup_old_logs(dir, prefix);

        let writer = RateLimitedFile::new(dir, prefix);

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

fn cleanup_old_logs(dir: &Path, prefix: &str) {
    let max_age = Duration::from_secs(MAX_LOG_AGE_DAYS * 24 * 60 * 60);
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !name.starts_with(prefix) || name == prefix {
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
