//! Provides logging details and functions

use clap::ValueEnum;
use std::str::FromStr;
use thiserror::Error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter::LevelFilter as TracingLevelFilter, fmt, Layer, Registry};

/// Initializes the logging framework for eventide dominion, using the default
/// configuration for [Logging](Logging)
pub fn init_logging(level: LevelFilter) {
    let layer = fmt::layer().with_filter::<TracingLevelFilter>(level.clone().into());

    Registry::default().with(layer).init()
}

/// Level filter
#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum LevelFilter {
    Trace,
    Debug,
    Info,
    Warn,
    Fatal,
    Off,
}

/// An error occurred while parsing a level filter
#[derive(Debug, Error)]
#[error("Unknown level: {unknown}")]
pub struct ParseError {
    unknown: String,
}

impl From<LevelFilter> for TracingLevelFilter {
    fn from(value: LevelFilter) -> Self {
        match value {
            LevelFilter::Trace => TracingLevelFilter::TRACE,
            LevelFilter::Debug => TracingLevelFilter::DEBUG,
            LevelFilter::Info => TracingLevelFilter::INFO,
            LevelFilter::Warn => TracingLevelFilter::WARN,
            LevelFilter::Fatal => TracingLevelFilter::ERROR,
            LevelFilter::Off => TracingLevelFilter::OFF,
        }
    }
}
