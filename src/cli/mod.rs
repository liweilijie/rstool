mod csv;
mod genpass;

use clap::Parser;
use std::path::Path;

pub use csv::OutputFormat;
pub use csv::CsvOpts;
pub use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name = "rstool", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

// rstool csv -i input.csv -o output.json -d ',' -h true
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv, or convert csv to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
