use clap::{value_parser, ArgAction, Args, Parser};
use shared::cli::Logging;
use std::path::PathBuf;

use shared::logging::LevelFilter;

/// The command line args for the server
#[derive(Debug, Parser)]
pub struct AppArgs {
    #[clap(flatten)]
    logging: Logging,
    /// Extra paths to static content to serve
    #[clap(long = "static")]
    pub static_content: Option<Vec<PathBuf>>,
}

impl AppArgs {
    pub fn level_filter(&self) -> LevelFilter {
        self.logging.level_filter()
    }
}
