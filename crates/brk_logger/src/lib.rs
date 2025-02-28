#![doc = include_str!("../README.md")]

use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use color_eyre::owo_colors::OwoColorize;
use env_logger::{Builder, Env};
use jiff::{Timestamp, tz};

#[inline(always)]
pub fn init(path: Option<&Path>) {
    let file = path.map(|path| {
        let _ = fs::remove_file(path);
        OpenOptions::new().create(true).append(true).open(path).unwrap()
    });

    Builder::from_env(Env::default().default_filter_or("info,fjall=off,lsm_tree=off"))
        .format(move |buf, record| {
            let date_time = Timestamp::now()
                .to_zoned(tz::TimeZone::system())
                .strftime("%Y-%m-%d %H:%M:%S")
                .to_string();
            let level = record.level().as_str().to_lowercase();
            let level = format!("{:5}", level);
            let target = record.target();
            let dash = "-";
            let args = record.args();

            if let Some(file) = file.as_ref() {
                let _ = write(file.try_clone().unwrap(), &date_time, target, &level, dash, args);
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

            write(buf, colored_date_time, target, colored_level, colored_dash, args)
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
    // writeln!(buf, "{} {} {} {}  {}", date_time, _target, level, dash, args)
}
