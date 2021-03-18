//! Logger module for customize logs

use chrono::Local;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::Level;
use std::io::Write;

/// Initialize logger
pub fn init(level: String) {
    let level = match &*level {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "Error" => Level::Error,
        &_ => Level::Error,
    };

    Builder::new()
        .format(move |buf, record| {
            let mut level_style = buf.style();

            let (color, level_spaces) = match record.level() {
                Level::Trace => (Color::White, " "),
                Level::Debug => (Color::Green, " "),
                Level::Info => (Color::Blue, "  "),
                Level::Warn => (Color::Yellow, "  "),
                Level::Error => (Color::Red, " "),
            };

            level_style.set_color(color).set_bold(true);
            let line = match record.line() {
                Some(line) => format!(":{}", line),
                None => "".to_owned(),
            };

            writeln!(
                buf,
                "{} [{}]{}{}{} | {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level_style.value(record.level()),
                level_spaces,
                record.target(),
                line,
                record.args()
            )
        })
        .filter(None, level.to_level_filter())
        .init();
}
