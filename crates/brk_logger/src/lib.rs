#![doc = include_str!("../README.md")]

use std::{fmt::Write as _, io, path::Path, sync::OnceLock};

use jiff::{Timestamp, tz};
use logroller::{LogRollerBuilder, Rotation, RotationSize};
use owo_colors::OwoColorize;
use tracing::{Event, Level, Subscriber, field::Field};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, FmtContext, FormatEvent, FormatFields, format::Writer},
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

type LogHook = Box<dyn Fn(&str) + Send + Sync>;

static GUARD: OnceLock<WorkerGuard> = OnceLock::new();
static LOG_HOOK: OnceLock<LogHook> = OnceLock::new();

const MAX_LOG_FILES: u64 = 5;
const MAX_FILE_SIZE_MB: u64 = 42;

// Don't remove, used to know the target of unwanted logs
const WITH_TARGET: bool = false;
// const WITH_TARGET: bool = true;

const fn level_str(level: Level) -> &'static str {
    match level {
        Level::ERROR => "error",
        Level::WARN => "warn ",
        Level::INFO => "info ",
        Level::DEBUG => "debug",
        Level::TRACE => "trace",
    }
}

struct Formatter<const ANSI: bool>;

/// Visitor that collects structured fields for colored formatting
struct FieldVisitor<const ANSI: bool> {
    result: String,
    status: Option<u64>,
    uri: Option<String>,
    latency: Option<String>,
}

impl<const ANSI: bool> FieldVisitor<ANSI> {
    fn new() -> Self {
        Self {
            result: String::new(),
            status: None,
            uri: None,
            latency: None,
        }
    }

    fn finish(self) -> String {
        // Format HTTP-style log if we have status
        if let Some(status) = self.status {
            let status_str = if ANSI {
                match status {
                    200..=299 => status.green().to_string(),
                    300..=399 => status.bright_black().to_string(),
                    _ => status.red().to_string(),
                }
            } else {
                status.to_string()
            };

            let uri = self.uri.as_deref().unwrap_or("");
            let latency = self.latency.as_deref().unwrap_or("");

            if ANSI {
                format!("{status_str} {uri} {}", latency.bright_black())
            } else {
                format!("{status_str} {uri} {latency}")
            }
        } else {
            self.result
        }
    }
}

impl<const ANSI: bool> tracing::field::Visit for FieldVisitor<ANSI> {
    fn record_u64(&mut self, field: &Field, value: u64) {
        let name = field.name();
        if name == "status" {
            self.status = Some(value);
        } else if !name.starts_with("log.") {
            let _ = write!(self.result, "{}={} ", name, value);
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        let name = field.name();
        if !name.starts_with("log.") {
            let _ = write!(self.result, "{}={} ", name, value);
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        let name = field.name();
        if name == "uri" {
            self.uri = Some(value.to_string());
        } else if name == "message" {
            let _ = write!(self.result, "{value}");
        } else if !name.starts_with("log.") {
            let _ = write!(self.result, "{}={} ", name, value);
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let name = field.name();
        match name {
            "uri" => self.uri = Some(format!("{value:?}")),
            "latency" => self.latency = Some(format!("{value:?}")),
            "message" => {
                let _ = write!(self.result, "{value:?}");
            }
            _ if name.starts_with("log.") => {}
            _ => {
                let _ = write!(self.result, "{}={:?} ", name, value);
            }
        }
    }
}

impl<S, N, const ANSI: bool> FormatEvent<S, N> for Formatter<ANSI>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let ts = Timestamp::now()
            .to_zoned(tz::TimeZone::system())
            .strftime("%Y-%m-%d %H:%M:%S")
            .to_string();

        let level = *event.metadata().level();
        let level_str = level_str(level);

        if ANSI {
            let level_colored = match level {
                Level::ERROR => level_str.red().to_string(),
                Level::WARN => level_str.yellow().to_string(),
                Level::INFO => level_str.green().to_string(),
                Level::DEBUG => level_str.blue().to_string(),
                Level::TRACE => level_str.cyan().to_string(),
            };
            if WITH_TARGET {
                write!(
                    writer,
                    "{} {} {} {level_colored} ",
                    ts.bright_black(),
                    event.metadata().target(),
                    "-".bright_black(),
                )?;
            } else {
                write!(
                    writer,
                    "{} {} {level_colored} ",
                    ts.bright_black(),
                    "-".bright_black()
                )?;
            }
        } else if WITH_TARGET {
            write!(writer, "{ts} {} - {level_str} ", event.metadata().target())?;
        } else {
            write!(writer, "{ts} - {level_str} ")?;
        }

        let mut visitor = FieldVisitor::<ANSI>::new();
        event.record(&mut visitor);
        write!(writer, "{}", visitor.finish())?;
        writeln!(writer)
    }
}

struct HookLayer;

impl<S: Subscriber> tracing_subscriber::Layer<S> for HookLayer {
    fn on_event(&self, event: &Event<'_>, _: tracing_subscriber::layer::Context<'_, S>) {
        if let Some(hook) = LOG_HOOK.get() {
            let mut msg = String::new();
            event.record(&mut MessageVisitor(&mut msg));
            hook(&msg);
        }
    }
}

struct MessageVisitor<'a>(&'a mut String);

impl tracing::field::Visit for MessageVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write;
        if field.name() == "message" {
            self.0.clear();
            let _ = write!(self.0, "{value:?}");
        }
    }
}

pub fn init(path: Option<&Path>) -> io::Result<()> {
    // Bridge log crate to tracing (for vecdb and other log-based crates)
    tracing_log::LogTracer::init().ok();

    #[cfg(debug_assertions)]
    const DEFAULT_LEVEL: &str = "debug";
    #[cfg(not(debug_assertions))]
    const DEFAULT_LEVEL: &str = "info";

    let default_filter = format!(
        "{DEFAULT_LEVEL},bitcoin=off,bitcoincore-rpc=off,fjall=off,brk_fjall=off,lsm_tree=off,brk_rolldown=off,rolldown=off,tracing=off,aide=off,rustls=off,notify=off,oxc_resolver=off,tower_http=off"
    );

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().event_format(Formatter::<true>))
        .with(HookLayer);

    if let Some(path) = path {
        let dir = path.parent().unwrap_or(Path::new("."));
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("app.log");

        let roller = LogRollerBuilder::new(dir, Path::new(filename))
            .rotation(Rotation::SizeBased(RotationSize::MB(MAX_FILE_SIZE_MB)))
            .max_keep_files(MAX_LOG_FILES)
            .build()
            .map_err(io::Error::other)?;

        let (non_blocking, guard) = tracing_appender::non_blocking(roller);
        GUARD.set(guard).ok();

        registry
            .with(
                fmt::layer()
                    .event_format(Formatter::<false>)
                    .with_writer(non_blocking),
            )
            .init();
    } else {
        registry.init();
    }

    Ok(())
}

/// Register a hook that gets called for every log message.
/// Can only be called once.
pub fn register_hook<F>(hook: F) -> Result<(), &'static str>
where
    F: Fn(&str) + Send + Sync + 'static,
{
    LOG_HOOK
        .set(Box::new(hook))
        .map_err(|_| "Hook already registered")
}
