use std::error::Error;
use std::path::PathBuf;
use clap::Parser;

/// Program which streams through a gzipped CSV, greps for rows which match some criteria and then
/// streams those rows to a new file on S3
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
