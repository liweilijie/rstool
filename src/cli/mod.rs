mod base64;
mod csv;
mod genpass;
mod text;

use clap::Parser;
use std::path::Path;
use std::path::PathBuf;

pub use base64::Base64Format;
pub use base64::Base64SubCommand;
pub use csv::CsvOpts;
pub use csv::OutputFormat;
pub use genpass::GenPassOpts;
pub use text::TextSignFormat;
pub use text::TextSubCommand;

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
    #[command(subcommand, about = "Sign or verify a text")]
    Text(TextSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
