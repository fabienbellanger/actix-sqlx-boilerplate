//! Logger module for customize logs

use chrono::Local;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::Level;
use std::io::Write;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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

pub fn init_tracing(level: String) {
    let file_appender = tracing_appender::rolling::daily("./logs", "axtix-sqlx.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // let subscriber = tracing_subscriber::registry()
    //     // TODO: use level variable
    //     .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
    //     .with(fmt::Layer::new().with_writer(std::io::stdout));
    // // .with(fmt::Layer::new().with_writer(non_blocking));
    // tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");

    // tracing_subscriber::fmt()
    //     .json()
    //     .with_max_level(tracing::Level::TRACE)
    //     .with_current_span(false)
    //     .with_timer(tracing_subscriber::fmt::time::ChronoLocal::with_format(String::from(
    //         "%Y-%m-%dT%H:%M:%S",
    //     )))
    //     .init();

    // TODO: Create own format layer
    let fmt_layer = fmt::layer().with_target(true).with_level(true).with_timer(
        tracing_subscriber::fmt::time::ChronoLocal::with_format(String::from("%Y-%m-%dT%H:%M:%S")),
    );
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
    .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Layer::new().with_writer(non_blocking))
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{FormatEvent, FormatFields, FmtContext, FormattedFields};
use tracing_subscriber::registry::LookupSpan;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        writer: &mut dyn std::fmt::Write,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        // Write level and target
        let level = *event.metadata().level();
        let target = event.metadata().target();
        write!(
            writer,
            "{} {}: ",
            level,
            target,
        )?;

        // Write spans and fields of each span
        ctx.visit_spans(|span| {
            write!(writer, "{}", span.name())?;

            let ext = span.extensions();

            // `FormattedFields` is a a formatted representation of the span's
            // fields, which is stored in its extensions by the `fmt` layer's
            // `new_span` method. The fields will have been formatted
            // by the same field formatter that's provided to the event
            // formatter in the `FmtContext`.
            let fields = &ext
                .get::<FormattedFields<N>>()
                .expect("will never be `None`");

            if !fields.is_empty() {
                write!(writer, "{{{}}}", fields)?;
            }
            write!(writer, ": ")?;

            Ok(())
        })?;

        // Write fields on the event
        ctx.field_format().format_fields(writer, event)?;

        writeln!(writer)
    }
}
