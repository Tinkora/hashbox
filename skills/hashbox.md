# hashbox Agent Skill

A browser-native multi-algorithm hashing toolbox. All computation runs in WASM. Privacy-first: zero uploads.

## Workflow

1. **Open Tool**: Navigate to the hashbox URL or start locally.
2. **Select Algorithm**: Choose SHA-256 (default), SHA-384, SHA-512, MD5, or BLAKE3.
3. **Input Data**: Type text (live hashing) or drag-and-drop a file.
4. **View Result**: Hash appears instantly; click copy to send to clipboard.
5. **Optional HMAC**: Enter key + message for HMAC-SHA-256 or HMAC-SHA-512.

## Tool Definitions

### `hash_text`

Hash a text string with the specified algorithm.

**Parameters:**
- `algorithm` (string, required): Algorithm key — `"sha256"`, `"sha384"`, `"sha512"`, `"md5"`, `"blake3"`
- `text` (string, required): The UTF-8 text to hash

**Returns:**
- `hex` (string): Lowercase hex digest

### `hash_bytes`

Hash raw bytes with the specified algorithm.

**Parameters:**
- `algorithm` (string, required): Algorithm key
- `bytes` (base64 string, required): Raw bytes encoded as base64

**Returns:**
- `hex` (string): Lowercase hex digest

### `hmac`

Compute HMAC for the given algorithm, key, and message.

**Parameters:**
- `algorithm` (string, required): `"sha256"` or `"sha512"`
- `key` (string, required): Secret key (non-empty)
- `message` (string, required): Message text

**Returns:**
- `hex` (string): Lowercase hex HMAC digest

### `list_algorithms`

List all supported algorithms with metadata.

**Returns:**
- `algorithms` (array): Array of `{ key, label, outputLen }`

## Agent Rules

- Never claim MD5 is secure — it is for legacy checksums only.
- Never ask users to upload files or text to a server — all hashing is local.
- The tool is fully static; no server-side processing exists.
- For large files (>100MB), recommend the streaming `HashWriter` API.
- HMAC keys must be non-empty.
