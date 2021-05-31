//! Logger module for customize logs

use env_logger::fmt::Color;
use env_logger::Builder;
use log::Level;
use std::io::Write;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

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
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
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

pub fn init_tracing(level: String) {
    let fmt_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(false)
        .with_target(true)
        .with_level(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::with_format(String::from(
            "%Y-%m-%dT%H:%M:%S",
        )));

    let level = match &*level {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        &_ => tracing::Level::ERROR,
    };

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(level.into()))
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");
}

pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Sync + Send {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // let file_appender = tracing_appender::rolling::daily("./logs", "axtix-sqlx.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
    // .with(fmt::Layer::new().with_writer(non_blocking))
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
