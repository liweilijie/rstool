use std::{fmt, path::Path, str::FromStr};
use clap::{Parser};

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

/*
 *   当您在 match 语句中使用 format 时，Rust 需要能够复制这个值，因为：
 *   所有权转移：在 match 语句中，format 的所有权会被转移到每个分支
 *   多次使用：如果 format 没有 Copy trait，它只能被使用一次，但 match 语句可能需要多次访问这个值
 *   编译时检查：Rust 编译器需要确保类型可以在编译时被复制
 *   通过为 OutputFormat 添加 #[derive(Copy, Clone)]，您告诉 Rust：
 *   Copy：这个类型可以通过简单的内存复制来复制（适用于小型的、简单的类型）
 *   Clone：这个类型可以通过调用 clone() 方法来复制
 *   由于 OutputFormat 是一个简单的枚举（只包含两个变体，没有关联数据），它非常适合使用 Copy trait。
 */
#[derive(Debug, Parser, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse::<OutputFormat>()
}

// 定义了一个将 OutputFormat 枚举类型转换为字符串的过程。
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}
