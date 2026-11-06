use std::{
    io::ErrorKind,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Context};
use doseark_db_postgres::create_pg_pool;
use sqlx::PgPool;
use tokio::time::{sleep, timeout};

use crate::config::ApiConfig;

pub async fn connect(config: &ApiConfig) -> anyhow::Result<PgPool> {
    let total_attempts = config.database_connect_attempts;

    for attempt in 1..=total_attempts {
        let started_at = Instant::now();

        tracing::info!(
            attempt,
            total_attempts,
            target = %config.database_target(),
            timeout_ms = duration_millis(config.database_connect_timeout),
            "Connecting to database"
        );

        let connection = timeout(
            config.database_connect_timeout,
            create_pg_pool(&config.database_url, config.database_max_connections),
        )
        .await;

        match connection {
            Ok(Ok(pool)) => {
                tracing::info!(
                    elapsed_ms = duration_millis(started_at.elapsed()),
                    max_connections = config.database_max_connections,
                    "Database connection established"
                );
                return Ok(pool);
            }
            Ok(Err(error)) => {
                let is_last_attempt = attempt == total_attempts;
                log_connection_failure(config, attempt, total_attempts, &error, is_last_attempt);

                if is_last_attempt {
                    return Err(error).with_context(|| {
                        format!(
                            "could not connect to database at {}",
                            config.database_target()
                        )
                    });
                }
            }
            Err(_) => {
                let is_last_attempt = attempt == total_attempts;
                let timeout_ms = duration_millis(config.database_connect_timeout);
                let hint = "Check IP/port, firewall, routing, listen_addresses, and pg_hba.conf.";

                if is_last_attempt {
                    tracing::error!(
                        attempt,
                        total_attempts,
                        timeout_ms,
                        hint,
                        "Database connection timed out"
                    );
                    return Err(anyhow!(
                        "database connection timed out after {timeout_ms} ms at {}",
                        config.database_target()
                    ));
                }

                tracing::warn!(
                    attempt,
                    total_attempts,
                    timeout_ms,
                    retry_in_ms = duration_millis(config.database_retry_delay),
                    hint,
                    "Database connection timed out; retrying"
                );
            }
        }

        sleep(config.database_retry_delay).await;
    }

    unreachable!("database connection loop must return on success or final failure")
}

fn log_connection_failure(
    config: &ApiConfig,
    attempt: u32,
    total_attempts: u32,
    error: &sqlx::Error,
    is_last_attempt: bool,
) {
    let hint = connection_hint(error);

    if is_last_attempt {
        tracing::error!(
            attempt,
            total_attempts,
            reason = %error,
            hint,
            "Database connection failed"
        );
    } else {
        tracing::warn!(
            attempt,
            total_attempts,
            retry_in_ms = duration_millis(config.database_retry_delay),
            reason = %error,
            hint,
            "Database connection failed; retrying"
        );
    }
}

fn connection_hint(error: &sqlx::Error) -> &'static str {
    match error {
        sqlx::Error::Io(io_error) => match io_error.kind() {
            ErrorKind::TimedOut => {
                "Host did not answer. Check IP/port, firewall, routing, listen_addresses, and pg_hba.conf."
            }
            ErrorKind::ConnectionRefused => {
                "PostgreSQL refused the connection. Check the service and published port 5432."
            }
            ErrorKind::NotFound => "Check the database hostname in DATABASE_URL.",
            _ => "Check network access and DATABASE_URL.",
        },
        sqlx::Error::Database(database_error) => match database_error.code().as_deref() {
            Some("28P01") => "PostgreSQL rejected the username or password in DATABASE_URL.",
            Some("28000") => "PostgreSQL rejected authorization for this user.",
            Some("3D000") => "The database name from DATABASE_URL does not exist.",
            Some("42501") => "The PostgreSQL user does not have the required permissions.",
            _ => "PostgreSQL answered with an error. Check the server log and DATABASE_URL settings.",
        },
        sqlx::Error::Tls(_) => {
            "Check PostgreSQL TLS settings and DATABASE_URL sslmode."
        }
        sqlx::Error::PoolTimedOut => {
            "Check database load and connection-pool limits."
        }
        _ => "Check DATABASE_URL, PostgreSQL availability, credentials, and server logs.",
    }
}

fn duration_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
