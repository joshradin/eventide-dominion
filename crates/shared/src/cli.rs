//! Command line interface shared happiness

use crate::logging::LevelFilter;
use clap::{value_parser, ArgAction, Args};

#[derive(Debug, Args)]
pub struct Logging {
    /// Sets the level of the logs using
    #[clap(long)]
    #[clap(conflicts_with_all(["v", "q"]))]
    level: Option<LevelFilter>,
    /// Increases the verbosity of the logger
    ///
    /// Exclusive with the quiet and level options
    #[clap(short)]
    #[clap(conflicts_with_all(["level", "q"]))]
    #[clap(action = ArgAction::Count)]
    #[clap(value_parser = value_parser!(u8).range(0..=2))]
    v: u8,
    /// Decreases the verbosity of the logger
    ///
    /// Exclusive with the verbose and level options
    #[clap(short)]
    #[clap(conflicts_with_all(["level", "v"]))]
    #[clap(action = ArgAction::Count)]
    #[clap(value_parser = value_parser!(u8).range(0..=3))]
    q: u8,
}

impl Logging {
    pub fn level_filter(&self) -> LevelFilter {
        if let Some(filter) = self.level {
            return filter;
        }
        match self.v as i16 - self.q as i16 {
            -3 => LevelFilter::Off,
            -2 => LevelFilter::Fatal,
            -1 => LevelFilter::Warn,
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            2 => LevelFilter::Trace,
            _ => {
                unreachable!("clap should handle this incorrect value")
            }
        }
    }
}
