use std::fmt;

use anyhow::{anyhow, Context};
use chrono::Local;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::config::env_or_legacy;

const DEFAULT_ENV_FILTER: &str =
    "warn,doseark_api=debug,doseark_db_postgres=debug,doseark_db_sqlite=debug,tower_http=info,sqlx=warn";

#[derive(Debug, Clone, Copy)]
struct ConsoleLocalTimer;

impl FormatTime for ConsoleLocalTimer {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        write!(writer, "{}", Local::now().format("%H:%M:%S%.3f"))
    }
}

/// Initializes API logging from environment configuration only.
///
/// `RUST_LOG` controls filtering. If it is missing, DoseArk uses a conservative
/// development default. The UI does not modify log levels at runtime.
pub fn init_logging() -> anyhow::Result<WorkerGuard> {
    let (filter_text, source) = match std::env::var("RUST_LOG") {
        Ok(value) if !value.trim().is_empty() => (value, "RUST_LOG"),
        Ok(_) => {
            return Err(anyhow!(
                "RUST_LOG is set but empty; provide a valid tracing filter or remove the variable"
            ));
        }
        Err(std::env::VarError::NotPresent) => (DEFAULT_ENV_FILTER.to_string(), "built-in default"),
        Err(error) => return Err(anyhow!("could not read RUST_LOG: {error}")),
    };

    let env_filter = EnvFilter::try_new(&filter_text)
        .with_context(|| format!("invalid RUST_LOG tracing filter: {filter_text}"))?;

    let file_appender = tracing_appender::rolling::daily("logs/api", "doseark-api.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_timer(ConsoleLocalTimer)
        .with_target(env_flag(
            "DOSEARK_LOG_CONSOLE_TARGET",
            "MEDTAKER_LOG_CONSOLE_TARGET",
            false,
        )?)
        .with_thread_ids(false)
        .with_thread_names(false);

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .try_init()
        .context("could not initialize tracing subscriber")?;

    tracing::info!(
        logging_source = source,
        logging_filter = %filter_text,
        "API logging initialized"
    );

    Ok(guard)
}

fn env_flag(primary: &str, legacy: &str, default: bool) -> anyhow::Result<bool> {
    let Some(value) = env_or_legacy(primary, legacy) else {
        return Ok(default);
    };

    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(anyhow!(
            "invalid value for {primary}: '{value}'; expected true/false"
        )),
    }
}
