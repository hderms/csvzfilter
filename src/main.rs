use crate::config::Args;
use clap::Parser;
use csv::WriterBuilder;
use flate2::bufread::MultiGzDecoder;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufReader;

mod config;

fn main() -> anyhow::Result<()> {
    let config: Args = Args::parse();

    //file handling
    let file_path = config.file;
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);

    //uncompress and then parse
    let decoder = MultiGzDecoder::new(buf_reader);

    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(decoder);

    let headers = { csv_rdr.headers()?.clone() };

    let enumerated_headers: Vec<(usize, &str)> = if let Some(headers_to_filter) = config.columns {
        headers
            .iter()
            .enumerate()
            .filter(|(_, header_str)| headers_to_filter.contains(&String::from(*header_str)))
            .collect()
    } else {
        headers.iter().enumerate().collect()
    };

    let pattern = Regex::new(&config.pattern)?;

    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(io::stdout());
    writer.write_record(&headers)?;

    for result in csv_rdr.records() {
        let record = result?;
        for (i, _) in enumerated_headers.iter() {
            let value = record.get(*i);
            if let Some(v) = value {
                if pattern.is_match(v) {
                    writer.write_record(&record)?;
                }
            }
        }
    }
    writer.flush()?;

    Ok(())
}
