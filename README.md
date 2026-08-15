# hashbox — 在线哈希工具箱

[![CI](https://github.com/Tinkora/hashbox/actions/workflows/test.yml/badge.svg)](https://github.com/Tinkora/hashbox/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

浏览器原生的多算法哈希工具箱。支持 SHA-256、SHA-384、SHA-512、MD5、BLAKE3 及 HMAC，所有计算在 WASM 中完成，数据绝不上传。

## ✨ 功能特性

- 🔢 **多算法支持** — SHA-256 / SHA-384 / SHA-512 / MD5 / BLAKE3
- 📝 **实时文本哈希** — 输入即计算，无需点击按钮
- 📁 **文件哈希** — 拖放文件，浏览器本地计算哈希值
- 🔐 **HMAC 消息认证** — 支持 HMAC-SHA-256 和 HMAC-SHA-512
- 🔒 **隐私优先** — 所有计算在浏览器 WASM 中完成，数据不会离开你的设备
- ⚡ **高性能** — Rust → WASM，接近原生速度

## 🚀 快速开始

```bash
# 克隆仓库
git clone https://github.com/Tinkora/hashbox.git
cd hashbox

# 构建 Web WASM
wasm-pack build --target web crates/hashbox_web

# 启动本地服务器
cp crates/hashbox_web/pkg/* crates/hashbox_web/static/pkg/
cd crates/hashbox_web/static && python3 -m http.server 8080
```

浏览器打开 `http://localhost:8080`。

## 📂 项目结构

| 组件 | 描述 | 状态 |
|------|------|------|
| `hashbox_core` | 哈希算法实现、HMAC、错误类型 | ✅ |
| `hashbox_web` | WASM 桥接 + HTML 工具界面 | ✅ |
| `skills/` | Agent Skill 定义（MCP tools） | ✅ |

## 🔧 开发

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p hashbox_web --target wasm32-unknown-unknown
```

## 📄 文档

- [产品规格 (zh-CN)](docs/product_spec.zh-CN.md)

## 🤝 社区

- [贡献指南](./CONTRIBUTING.md)
- [行为准则](./CODE_OF_CONDUCT.md)
- [安全策略](./SECURITY.md)
- [更新日志](./CHANGELOG.md)

## Support

If hashbox saves you time, support Tinkora on [Ko-fi](https://ko-fi.com/tinkora).
Support is optional and never affects access or issue priority.

See [SUPPORT.md](./SUPPORT.md) for questions, bug reports, and security reports.

## 📜 许可

MIT © [Tinkora](https://github.com/Tinkora)
