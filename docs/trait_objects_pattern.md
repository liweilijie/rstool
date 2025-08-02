# Rust Trait Objects 设计模式

## 核心概念

### Box<dyn Read> 的作用
```rust
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))  // Stdin
    } else {
        Ok(Box::new(File::open(input)?))  // File
    }
}
```

## 设计模式解析

### 1. **Trait Objects 本质**
- `dyn Read` 表示"任何实现了 Read trait 的类型"
- `Box<dyn Read>` 是智能指针，指向堆上的 trait object
- 提供运行时多态性

### 2. **为什么需要 Box？**
```rust
// ❌ 错误：trait objects 不能直接使用
fn get_reader(input: &str) -> Result<dyn Read> {
    // 编译错误：trait objects 需要动态分发
}

// ✅ 正确：使用 Box 包装
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 可以工作：Box 提供了固定大小的指针
}
```

### 3. **统一接口的优势**
```rust
// 调用者不需要知道具体类型
let mut reader = get_reader(input)?;
reader.read_to_end(&mut buf)?;  // 统一的方法调用
```

## 内存布局

```
Box<dyn Read> 结构：
┌─────────────────┐
│   Box 指针      │ → 指向堆上的数据
└─────────────────┘
                    ┌─────────────────┐
                    │   vtable 指针   │ → 函数表
                    ├─────────────────┤
                    │   数据指针      │ → 实际数据
                    └─────────────────┘
```

## 性能考虑

### 动态分发开销
- 每次方法调用都需要通过 vtable 查找
- 比编译时多态慢，但提供运行时灵活性

### 何时使用
- ✅ 需要运行时多态
- ✅ 类型集合有限
- ❌ 高性能循环
- ❌ 编译时类型已知

## 替代方案

### 1. 泛型方式
```rust
fn get_reader_generic<T: Read + 'static>(input: &str) -> Result<T> {
    // 编译时确定类型，性能更好
}
```

### 2. 枚举方式
```rust
enum Reader {
    Stdin(std::io::Stdin),
    File(File),
}
```

### 3. Trait Objects 方式
```rust
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 当前使用的方式，灵活但有性能开销
}
```

## 实际应用

### 插件系统
```rust
trait Plugin {
    fn execute(&self) -> Result<()>;
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}
```

### 配置系统
```rust
trait ConfigSource {
    fn load(&self) -> Result<Config>;
}
```

## 总结

`Box<dyn Read>` 模式提供：
1. **统一接口**：不同实现者使用相同接口
2. **运行时多态**：运行时选择具体实现
3. **代码复用**：一套逻辑处理多种类型
4. **扩展性**：容易添加新实现者

这是 Rust 中实现多态性的重要方式！ 
