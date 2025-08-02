use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use super::verify_file;
use super::verify_path;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(
        name = "sign",
        about = "Sign a text with a private/session key and return a signature"
    )]
    Sign(TextSignOpts),
    #[command(
        name = "verify",
        about = "Verify a signature with a public/session key"
    )]
    Verify(TextVerifyOpts),
    #[command(
        name = "generate",
        about = "Generate a random blake3 key or ed25519 key pair"
    )]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!(format!("Invalid text sign format: {}", s))),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
