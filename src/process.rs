use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use csv::Reader;
use crate::opts::OutputFormat;

/// 球员数据结构体
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/// 将CSV文件转换为指定格式
pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value: serde_json::Value = headers.iter().zip(record.iter())
            .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
            .collect();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
