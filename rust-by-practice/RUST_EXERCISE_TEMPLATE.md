# Rust 练习题项目生成指南

## 项目结构

每次从网页提取时，根据 URL 路径确定嵌套文件夹结构。URL 中的每一级路径对应一级文件夹，最后一级是独立的 Cargo 项目。例如：
- `variables.html` → `variables/`（单级，直接是 Cargo 项目）
- `basic-types/numbers.html` → `basic-types/numbers/`（多级，逐层建文件夹，最后一级是 Cargo 项目）
- `flow-control/if-else.html` → `flow-control/if-else/`

```
rust-by-practice/
├── variables/                ← 单级 URL：直接是 Cargo 项目
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs           ← 说明注释 + 运行方式
│   │   └── main.rs           ← 空文件（仅含 fn main() {}）
│   └── tests/
│       ├── ex01.rs
│       └── ...
├── basic-types/              ← 多级 URL：先建父级文件夹
│   └── numbers/             ← 再建子级文件夹，这是 Cargo 项目
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   └── main.rs
│       └── tests/
│           ├── ex01.rs
│           └── ...
├── flow-control/
│   └── if-else/
│       └── ...
└── RUST_EXERCISE_TEMPLATE.md
```

多级路径时，父级文件夹只是组织分类用，最后一级才是 Cargo 项目。

## 文件内容规范

### src/lib.rs

```rust
// Rust 练习题 - <主题名称>
// 来源: <网页URL>
//
// 运行方式:
//   cargo test --test ex01   只跑第1题
//   cargo test --test ex02   只跑第2题
//   ...
//   cargo test               跑所有题
```

不需要其他代码。

### src/main.rs

```rust
fn main() {}
```

### tests/exNN.rs（每道题一个文件）

```rust
// <题号>. <星级> <题目说明文字>
// <补充说明或提示，如果有>

#[test]
fn ex<NN>() {
    <网页中的原始代码，原样保留，包含错误和填空>
}
```

## 关键规则

1. **每道题独立文件**：放在 `tests/` 下，命名为 `ex01.rs` `ex02.rs` ...，这样每题是独立 crate，互不影响
2. **代码原样保留**：代码内容与网页完全一致，故意不修复错误、不填空，留给做题者解决
3. **题目说明以注释保留**：星级、说明、提示等原文放在文件顶部注释中
4. **题号从01开始**：使用两位数编号（ex01, ex02, ...）保持排序整齐
5. **网页中有辅助函数的**：与题目放在同一个测试文件中（如 ex04 旁边的 `define_x` 函数）
6. **网页代码包含 `use crate::*` 的**：不能简单地把全部代码塞进 `fn exNN() {}` 里。`use crate::SomeType::*` 要求 `SomeType` 是 crate 级别的项，但函数内部的定义不是。解决办法：将类型定义、impl 块和 `use` 语句提升到测试文件模块级别（`fn exNN()` 之外），仅把 `fn main()` 及其内部逻辑保留在 `fn exNN()` 内。特别注意 `cargo init` 创建的项目名可能和 Rust 关键字冲突（如 `enum`），需要用 `--name` 指定非关键字名称

## 使用方式

提供网页 URL 和本文件给 AI，说：

> 请按照 RUST_EXERCISE_TEMPLATE.md 的格式，提取 <URL> 的内容生成练习题项目

AI 将执行：
1. 从 URL 路径提取嵌套文件夹名（如 `variables.html` → `variables`，`basic-types/numbers.html` → `basic-types/numbers`）
2. 用 WebFetch 抓取网页内容
3. 逐层创建文件夹（多级路径时先建父级，再建子级）
4. 在最后一级文件夹内用 `cargo init` 创建独立的 Cargo 项目（如果文件夹名是 Rust 关键字如 `enum`，需加 `--name` 指定别名）
5. 按规范将每道题写入 `tests/exNN.rs`
6. 写入 `src/lib.rs`（来源 + 运行说明）和 `src/main.rs`
7. 用 `cargo check` 验证项目结构正常