# Repository Guide for AI Agents

## Project Overview

hashbox is a browser-native multi-algorithm hashing toolbox. All computation runs in WASM compiled from Rust. The tool is privacy-first: no data is ever uploaded. Supports both text input and file hashing via FileReader API, plus HMAC message authentication.

## Architecture

```
hashbox/
├── crates/
│   ├── hashbox_core/       # Hash algorithms, HMAC, error types
│   └── hashbox_web/        # WASM bridge + HTML tool page
├── docs/                    # Product specs (Chinese)
├── skills/                  # Agent Skill definitions (MCP tools)
└── index.html               # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/hashbox_core/src/hash.rs` | HashAlgorithm enum, hash(), hash_streaming(), hmac_* |
| `crates/hashbox_core/src/error.rs` | CoreError enum with stable code() |
| `crates/hashbox_core/src/wasm.rs` | WASM bindings: wasm_hash, wasm_hash_text, wasm_hmac, wasm_list_algorithms |
| `crates/hashbox_core/src/lib.rs` | Crate root; re-exports |
| `crates/hashbox_web/src/lib.rs` | Thin cdylib re-export |
| `crates/hashbox_web/static/index.html` | Full-featured SPA tool with algorithm selector, text area, file drop zone, HMAC |
| `skills/hashbox.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |
| `docs/product_spec.zh-CN.md` | Product specification (Chinese) |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p hashbox_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/hashbox_web
```

## Design Principles

1. **Browser-first**: All hashing runs in WASM, zero network requests after initial page load
2. **Privacy-first**: No uploads. Text and files are processed entirely in-browser. No telemetry.
3. **Live feedback**: Text hashing is debounced (80ms) and updates as you type; file hashing triggers on drop/select
4. **Uniform API**: All five algorithms share the same `hash(alg, &[u8]) -> String` signature
5. **Streaming support**: `HashWriter` implements `std::io::Write` for incremental hashing of large inputs

## Supported Algorithms

| Algorithm | Key | Digest (bytes) | Hex Length | Library |
|-----------|-----|----------------|------------|---------|
| SHA-256 | `sha256` | 32 | 64 | sha2 0.10 |
| SHA-384 | `sha384` | 48 | 96 | sha2 0.10 |
| SHA-512 | `sha512` | 64 | 128 | sha2 0.10 |
| MD5 | `md5` | 16 | 32 | md-5 0.10 |
| BLAKE3 | `blake3` | 32 | 64 | blake3 1.5 |

## HMAC

- HMAC-SHA-256 and HMAC-SHA-512 are supported
- Key must be non-empty (`CoreError::HmacInvalidKey` otherwise)
- Uses hmac 0.12 + sha2 0.10

## Error Codes (Stable Machine-Readable)

| Code | Meaning |
|------|---------|
| `UNSUPPORTED_ALGORITHM` | Unknown algorithm key string |
| `HMAC_INVALID_KEY` | Key is empty |

## WASM JS API

```js
import init, { wasmHash, wasmHashText, wasmHmac, wasmListAlgorithms } from './pkg/hashbox_web.js';
await init();

// Text hashing
const hex = wasmHashText('sha256', 'hello world');

// File hashing (pass Uint8Array)
const fileHex = wasmHash('blake3', fileBytes);

// HMAC
const encoder = new TextEncoder();
const mac = wasmHmac('sha256', encoder.encode('secret'), encoder.encode('message'));

// List available algorithms
const algs = wasmListAlgorithms(); // [{ key, label, outputLen }, ...]
```

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
