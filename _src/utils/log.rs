use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write,
};

use chrono::Local;
use color_eyre::owo_colors::OwoColorize;
use env_logger::Env;

use crate::structs::Config;

#[inline(always)]
pub fn init_log() {
    let _ = fs::remove_file(Config::path_log());

    let file = Box::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(Config::path_log())
            .unwrap(),
    );

    env_logger::Builder::from_env(
        Env::default().default_filter_or(format!("{}=info", env!("CARGO_PKG_NAME"))),
    )
    .format(move |buf, record| {
        let date_time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        let level = record.level().as_str().to_lowercase();
        let level = format!("{:5}", level);
        let target = record.target();
        let dash = "-";
        let args = record.args();

        let _ = write(
            file.try_clone().unwrap(),
            &date_time,
            target,
            &level,
            dash,
            args,
        );

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
}

fn write(
    mut buf: impl Write,
    date_time: impl Display,
    _target: impl Display,
    level: impl Display,
    dash: impl Display,
    args: impl Display,
) -> Result<(), std::io::Error> {
    writeln!(buf, "{} {} {} {}", date_time, dash, level, args)
    // writeln!(
    //     buf,
    //     "{} {} {} {}  {}",
    //     date_time, _target, level, dash, args
    // )
}
