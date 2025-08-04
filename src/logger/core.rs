use std::{fmt, path::PathBuf};

use super::*;

// use serde::Deserialize;
use tracing_subscriber::{
    filter::{LevelFilter, Targets}, layer::SubscriberExt, Layer, Registry
};
use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
use clap::ValueEnum;
use chrono::Local;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::format::Writer;

#[derive(Debug)]
pub struct LogConfig {
    pub console_logger_level: LogLevel,
    pub file_logger_level: LogLevel,
    pub dir: PathBuf,
}

// #[derive(Debug, Deserialize, Clone, Copy)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
// #[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info  => LevelFilter::INFO,
            LogLevel::Warn  => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info  => "info",
            LogLevel::Warn  => "warn",
            LogLevel::Error => "error",
        };
        write!(f, "{}", s)
    }
}






// use time::{OffsetDateTime, format_description::well_known::Rfc3339};

struct LocalTimer;

// impl FormatTime for LocalTimer {
//     fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
//         let now = OffsetDateTime::now_local()
//             .unwrap_or_else(|_| OffsetDateTime::now_utc());
//         write!(w, "{}", now.format(&Rfc3339).unwrap())
//     }
// }

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let now = Local::now();
        // write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S"))
        write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S%.3f%:z"))
    }
}


pub fn init(log_config: &LogConfig) -> Result<WorkerGuard> {

    let log_config_rotation = Rotation::DAILY; // Rotation::HOURLY;
    let log_config_file_prefix = "log".to_string();

    let app_name = env!("CARGO_PKG_NAME").replace("-", "_");
    // let level_filter_string = format!("{}={}", app_name, log_config.level.to_string());

    let console_level_filter = LevelFilter::from(log_config.console_logger_level);
    let console_layer_targets = Targets::new().with_target(&app_name, console_level_filter);
    let console_layer = tracing_subscriber::fmt::layer()
        // .pretty()
        .with_timer(LocalTimer)
        .with_target(false)
        .with_ansi(true)
        .with_writer(std::io::stdout)
        .with_filter(console_layer_targets);

    let file_level_filter = LevelFilter::from(log_config.file_logger_level);
    let file_layer_targets = Targets::new().with_target(&app_name, file_level_filter);
    let file_appender = RollingFileAppender::new(log_config_rotation, &log_config.dir, &log_config_file_prefix);
    let (non_blocking_file_appender, file_log_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_timer(LocalTimer)
        .with_target(false)
        .with_ansi(false)
        .with_writer(non_blocking_file_appender)
        .with_filter(file_layer_targets.clone());

    let subscriber = Registry::default()
        // .with(EnvFilter::new(&level_filter_string))
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| Error::SetLogger(e))?;

    Ok(file_log_guard)

}


// pub fn flush(guard: WorkerGuard) {

//     // Allow background thread time to flush
//     std::thread::sleep(std::time::Duration::from_millis(500));

//     // Dropping the guard here will ensure the final flush happens
//     drop(guard);

// }