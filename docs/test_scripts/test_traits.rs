//! Trait 测试脚本
//! 
//! 这个文件包含了用于测试 FromStr, From, Display trait 的示例代码
//! 可以直接运行来验证 trait 实现是否正确

use std::str::FromStr;
use std::fmt;

// 模拟 OutputFormat 枚举
#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// 实现 FromStr trait
impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

// 实现 From trait
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 实现 Display trait
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

fn main() {
    println!("=== Trait 测试脚本 ===\n");

    // 测试 FromStr
    println!("1. 测试 FromStr trait:");
    test_from_str();

    // 测试 From
    println!("\n2. 测试 From trait:");
    test_from();

    // 测试 Display
    println!("\n3. 测试 Display trait:");
    test_display();

    // 测试 parse() 方法
    println!("\n4. 测试 parse() 方法:");
    test_parse();

    println!("\n=== 所有测试完成 ===");
}

fn test_from_str() {
    // 测试有效输入
    let json_format = OutputFormat::from_str("json").unwrap();
    let yaml_format = OutputFormat::from_str("yaml").unwrap();
    let json_upper = OutputFormat::from_str("JSON").unwrap();
    
    assert_eq!(json_format, OutputFormat::Json);
    assert_eq!(yaml_format, OutputFormat::Yaml);
    assert_eq!(json_upper, OutputFormat::Json);
    
    println!("  ✅ 有效输入测试通过");
    
    // 测试无效输入
    let invalid_result = OutputFormat::from_str("invalid");
    assert!(invalid_result.is_err());
    println!("  ✅ 无效输入测试通过");
}

fn test_from() {
    let json_format = OutputFormat::Json;
    let yaml_format = OutputFormat::Yaml;
    
    // 使用 into()
    let json_str: &'static str = json_format.clone().into();
    let yaml_str: &'static str = yaml_format.clone().into();
    
    assert_eq!(json_str, "json");
    assert_eq!(yaml_str, "yaml");
    println!("  ✅ into() 转换测试通过");
    
    // 使用 From::from()
    let json_str2 = <&'static str>::from(json_format);
    let yaml_str2 = <&'static str>::from(yaml_format);
    
    assert_eq!(json_str2, "json");
    assert_eq!(yaml_str2, "yaml");
    println!("  ✅ From::from() 转换测试通过");
}

fn test_display() {
    let json_format = OutputFormat::Json;
    let yaml_format = OutputFormat::Yaml;
    
    // 测试 println!
    println!("    JSON format: {}", json_format);
    println!("    YAML format: {}", yaml_format);
    
    // 测试 to_string()
    let json_string = json_format.to_string();
    let yaml_string = yaml_format.to_string();
    
    assert_eq!(json_string, "json");
    assert_eq!(yaml_string, "yaml");
    println!("  ✅ to_string() 测试通过");
}

fn test_parse() {
    // 测试 parse() 方法
    let format1: OutputFormat = "json".parse().unwrap();
    let format2 = "yaml".parse::<OutputFormat>().unwrap();
    
    assert_eq!(format1, OutputFormat::Json);
    assert_eq!(format2, OutputFormat::Yaml);
    println!("  ✅ parse() 方法测试通过");
    
    // 测试错误情况
    let invalid_parse = "invalid".parse::<OutputFormat>();
    assert!(invalid_parse.is_err());
    println!("  ✅ parse() 错误处理测试通过");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_traits() {
        test_from_str();
        test_from();
        test_display();
        test_parse();
    }
} 
