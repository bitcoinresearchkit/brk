use std::time::Instant;

use color_eyre::owo_colors::OwoColorize;

pub trait DisplayInstant {
    fn display(&self) -> String;
}

impl DisplayInstant for Instant {
    fn display(&self) -> String {
        format!("{:.2}s", self.elapsed().as_secs_f32())
            .bright_black()
            .to_string()
    }
}
