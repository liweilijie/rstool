mod base64;
mod csv;
mod genpass;

use clap::Parser;
use std::path::Path;

pub use base64::Base64Format;
pub use base64::Base64SubCommand;
pub use csv::CsvOpts;
pub use csv::OutputFormat;
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
    #[command(subcommand, about = "Encode or decode a base64 string")]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
