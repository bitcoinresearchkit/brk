use std::fmt::Write;

use jiff::{Timestamp, tz};
use owo_colors::OwoColorize;
use tracing::{Event, Level, Subscriber, field::Field};
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

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

pub struct Formatter<const ANSI: bool>;

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
