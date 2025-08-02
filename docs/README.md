# 项目文档

本目录包含 rstool 项目的详细文档。

## 文档列表

### [迭代器操作详解](./iterator_operations.md)
详细解释了 `process_csv` 函数中迭代器操作的使用，包括：
- `zip()` 操作的工作原理
- `map()` 转换过程
- `collect()` 收集机制
- 完整的代码示例和流程图

### [Trait 详解：From, FromStr, Display](./traits_explanation.md)
详细解释了 `opts.rs` 中 `OutputFormat` 枚举实现的三个重要 trait：
- `FromStr` trait 的作用和实现原理
- `From` trait 的类型转换机制
- `Display` trait 的格式化输出
- `parse()` 方法的工作原理
- 实际应用场景和最佳实践

## 测试脚本

### [测试脚本目录](./test_scripts/)
包含用于学习和测试的示例代码：
- `test_traits.rs` - 完整的 trait 测试脚本
- `README.md` - 测试脚本使用说明

## 如何查看文档

1. **在编辑器中打开**：直接打开对应的 `.md` 文件
2. **使用 Markdown 预览**：在支持 Markdown 预览的编辑器中查看
3. **在线查看**：如果项目托管在 GitHub 等平台，可以直接在网页上查看

## 如何运行测试脚本

```bash
# 运行 trait 测试脚本
cd docs/test_scripts
rustc test_traits.rs -o test_traits
./test_traits
```

## 文档更新

当代码发生变化时，请及时更新相关文档以保持同步。 
