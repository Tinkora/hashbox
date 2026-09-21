# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (current) | ✅ |

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.

Instead, email the project maintainer directly. You should receive a response
within 48 hours. We will work with you to understand the scope and coordinate
a fix and disclosure timeline.

### Scope

The following areas are within scope:

- WASM sandbox escapes
- Incorrect hash outputs (cryptographic correctness)
- HMAC key handling issues
- Side-channel leaks in WASM execution

### Out of Scope

- Issues already documented as known limitations (e.g., MD5 is not collision-resistant — this is by design)
- Theoretical attacks requiring physical access
- Issues in dependencies (please report upstream)

## Security Model

The hashbox project follows these security principles:

1. **Browser-local only**: All hashing computation runs in WASM inside the browser. No user data (text, files, keys) is sent to any server. After the initial page load, zero network requests are made.

2. **No telemetry**: The tool does not include any analytics, tracking, or error-reporting that would exfiltrate hashed content.

3. **Well-audited dependencies**: Hash primitives come from the widely-audited RustCrypto project (sha2, md-5, hmac) and the official blake3 crate.

4. **Clear cryptographic intent**: MD5 is clearly labeled as not collision-resistant and intended only for legacy checksums.
