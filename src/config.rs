use clap::Parser;
use std::path::PathBuf;

/// Program which streams through a gzipped CSV, greps for rows which match some criteria and then
/// streams those to stdout as plaintext CSV
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// path of the file
    #[arg(short, long)]
    pub file: PathBuf,

    /// pattern to search by
    #[arg(short, long)]
    pub pattern: String,

    /// fields we should search by name
    #[arg(short, long)]
    pub columns: Option<Vec<String>>,
}
