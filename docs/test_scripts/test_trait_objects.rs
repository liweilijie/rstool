//! Trait Objects 测试脚本
//! 
//! 演示 Box<dyn Read> 模式的使用和优势

use std::io::{Read, Write};
use std::fs::File;

// 模拟 get_reader 函数
fn get_reader(input: &str) -> Result<Box<dyn Read>, Box<dyn std::error::Error>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

// 统一的处理函数
fn process_reader(mut reader: Box<dyn Read>) -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

// 演示不同的输入源
fn demonstrate_trait_objects() {
    println!("=== Trait Objects 演示 ===\n");

    // 1. 从文件读取
    println!("1. 从文件读取:");
    match get_reader("Cargo.toml") {
        Ok(reader) => {
            match process_reader(reader) {
                Ok(content) => println!("   文件内容长度: {} 字符", content.len()),
                Err(e) => println!("   读取错误: {}", e),
            }
        }
        Err(e) => println!("   打开文件错误: {}", e),
    }

    // 2. 从字符串读取（模拟）
    println!("\n2. 从字符串读取:");
    let test_data = "Hello, Trait Objects!";
    let mut reader: Box<dyn Read> = Box::new(test_data.as_bytes());
    match process_reader(reader) {
        Ok(content) => println!("   内容: {}", content),
        Err(e) => println!("   读取错误: {}", e),
    }

    // 3. 演示多态性
    println!("\n3. 多态性演示:");
    let readers: Vec<Box<dyn Read>> = vec![
        Box::new("数据1".as_bytes()),
        Box::new("数据2".as_bytes()),
    ];

    for (i, mut reader) in readers.into_iter().enumerate() {
        let mut buf = String::new();
        if reader.read_to_string(&mut buf).is_ok() {
            println!("   读取器 {}: {}", i + 1, buf);
        }
    }
}

// 对比：传统方式 vs Trait Objects
fn compare_approaches() {
    println!("\n=== 方式对比 ===\n");

    // 传统方式：需要为每种类型写不同的函数
    println!("传统方式:");
    println!("  - 需要为每种输入源写不同的函数");
    println!("  - 代码重复");
    println!("  - 难以扩展");

    // Trait Objects 方式
    println!("\nTrait Objects 方式:");
    println!("  - 一个函数处理所有类型");
    println!("  - 代码复用");
    println!("  - 易于扩展");
}

// 性能考虑演示
fn performance_considerations() {
    println!("\n=== 性能考虑 ===\n");

    println!("Trait Objects 开销:");
    println!("  - 动态分发：每次方法调用通过 vtable 查找");
    println!("  - 内存分配：Box 需要堆分配");
    println!("  - 缓存不友好：间接调用");

    println!("\n适用场景:");
    println!("  ✅ 需要运行时多态");
    println!("  ✅ 类型集合有限");
    println!("  ✅ 性能不是关键因素");
    println!("  ❌ 高性能循环");
    println!("  ❌ 编译时类型已知");
}

fn main() {
    demonstrate_trait_objects();
    compare_approaches();
    performance_considerations();

    println!("\n=== 总结 ===");
    println!("Box<dyn Read> 模式提供:");
    println!("1. 统一接口");
    println!("2. 运行时多态");
    println!("3. 代码复用");
    println!("4. 易于扩展");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reader_file() {
        let reader = get_reader("Cargo.toml");
        assert!(reader.is_ok());
    }

    #[test]
    fn test_process_reader() {
        let data = "test data";
        let reader: Box<dyn Read> = Box::new(data.as_bytes());
        let result = process_reader(reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test data");
    }

    #[test]
    fn test_trait_objects_polymorphism() {
        let readers: Vec<Box<dyn Read>> = vec![
            Box::new("hello".as_bytes()),
            Box::new("world".as_bytes()),
        ];

        let mut results = Vec::new();
        for mut reader in readers {
            let mut buf = String::new();
            reader.read_to_string(&mut buf).unwrap();
            results.push(buf);
        }

        assert_eq!(results, vec!["hello", "world"]);
    }
} 
