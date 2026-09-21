# Changelog

## [0.1.0] - 2026-08-14

### Added
- `hashbox_core`: `HashAlgorithm` enum (SHA-256, SHA-384, SHA-512, MD5, BLAKE3)
- `hashbox_core`: `hash()` one-shot hashing, `hash_streaming()` incremental hashing via `HashWriter`
- `hashbox_core`: `hmac_sha256()` and `hmac_sha512()` with non-empty key validation
- `hashbox_core`: `CoreError` with stable machine-readable `code()` method
- `hashbox_core`: WASM bindings: `wasmHash`, `wasmHashText`, `wasmHmac`, `wasmListAlgorithms`
- `hashbox_web`: cdylib crate re-exporting WASM bindings
- `hashbox_web/static/index.html`: Full-featured SPA tool with:
  - Algorithm selector (5 algorithms with live switching)
  - Text input area with debounced (80ms) live hashing
  - File drop zone with drag-and-drop support and FileReader API
  - HMAC section with key + message inputs
  - Copy-to-clipboard on all result outputs
- Agent Skill definition (`skills/`)
- Landing page with Chinese UI
- CI workflow (native test, clippy, WASM check, wasm-pack build)
- Documentation: product spec, AGENTS.md guide
