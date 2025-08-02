# GitHub Actions 问题分析

## 主要问题

### 1. **Changelog 生成被注释掉**
```yaml
# - name: Generate a changelog
#   uses: orhun/git-cliff-action@v2
```

**问题**：changelog 生成步骤被注释，导致 release body 为空。

### 2. **标签格式不标准**
您的标签：`v1-add-csv`, `v1-csv-done`, `v1-csv-json-yaml`
期望格式：`v1.0.0`, `v1.1.0`, `v2.0.0`

### 3. **缺少二进制文件构建**
没有构建和上传 release 二进制文件。

## 解决方案

### 修复 build.yml
```yaml
name: build

on:
  push:
    branches: [main]
    tags: [v*]
  pull_request:
    branches: [main]

permissions:
  contents: write

jobs:
  build-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Check code format
        run: cargo fmt -- --check
      
      - name: Check the package
        run: cargo check --all
      
      - name: Run tests
        run: cargo test
      
      - name: Generate changelog
        uses: orhun/git-cliff-action@v2
        id: git-cliff
        if: startsWith(github.ref, 'refs/tags/')
        with:
          config: cliff.toml
          args: --latest --strip header
      
      - name: Build release
        if: startsWith(github.ref, 'refs/tags/')
        run: cargo build --release
      
      - name: Create release
        uses: softprops/action-gh-release@v1
        if: startsWith(github.ref, 'refs/tags/')
        with:
          body: ${{ steps.git-cliff.outputs.content }}
          files: target/release/rstool
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 创建标准标签
```bash
# 删除旧标签
git tag -d v1-add-csv v1-csv-done v1-csv-json-yaml
git push origin :refs/tags/v1-add-csv

# 创建新标签
git tag v1.0.0
git push origin v1.0.0
```

## 测试步骤
1. 修复 build.yml
2. 创建标准标签
3. 推送触发 Actions
4. 检查 GitHub release 页面 
