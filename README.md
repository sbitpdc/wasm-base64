# 🚀 wasm-base64 - 基于 Rust 和 WebAssembly 的高性能 Base64 编码解码库

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-1.0-blue?logo=webassembly)](https://webassembly.org/)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-green)](https://opensource.org/licenses/AGPL-3.0)

pub fn base64_to_uint8_array(base64_str: &str) -> Result<Uint8Array, JsValue> {
**wasm-base64** 是一个基于 Rust 和 WebAssembly 的高性能 Base64 编码解码库，主要处理这array_buffer_to_base64、base64_to_array_buffer、uint8_array_to_base64、array_buffer_to_base64_direct方法。因为在处理图片转换的时候，使用原生的javascript方法，如果图片很大，处理起来会很卡，特别是要处理多张图片时候，性能相当差，还有内存泄露隐患。这个库专为现代 Web 应用设计。相比纯 JavaScript 实现，它提供 3-4 倍的性能提升，特别适合处理大型数据或性能敏感场景。

## ✨ 核心特性

- ⚡️ **超高性能**：Rust 原生实现，比 JavaScript 快 3-4 倍
- 🌐 **浏览器原生支持**：编译为 WebAssembly，所有现代浏览器均可运行
- 🔧 **简单易用**：简洁的 API 设计，轻松集成到现有项目
- 🛡️ **内存安全**：Rust 的所有权系统保证内存安全
- 📦 **轻量级**：优化后的 Wasm 文件仅 30KB 左右

## 🚦 快速开始

### 安装依赖
创建Rust WebAssembly项目

cargo new hello_world --lib


- 首先要安装 cargo install wasm-bindgen-cli
- 然后安装  cargo install wasm-bindgen-cli

- 下面是打包命令：
  - cargo build --target wasm32-unknown-unknown
  - rustup target add wasm32-unknown-unknown
  - cargo build --target wasm32-unknown-unknown --release
  - wasm--bindgen target/wasm32-unknown-unknown/release/wasm_base64.wasm --out-dir ./extension/wasm --target web

最后把 ./extension/wasm 下面的所有文件都复制到需要用到的地方
