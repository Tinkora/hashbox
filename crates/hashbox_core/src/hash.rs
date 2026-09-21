//! Multi-algorithm hashing with a uniform API.
//!
//! All implementations delegate to well-audited libraries (sha2, md-5,
//! blake3, hmac) and return lowercase hex strings.

use crate::error::CoreError;
use sha2::{Sha256, Sha384, Sha512};
use std::io::Write;

/// Hash algorithms supported by hashbox.
///
/// Every variant maps to a widely-deployed cryptographic or checksum
/// primitive.  Use [`hash`] for one-shot hashing or [`HashWriter`] for
/// incremental / streaming hashing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256 (32-byte digest).  Default choice for general-purpose integrity.
    Sha256,
    /// SHA-384 (48-byte digest).  Longer output; resistant to length-extension.
    Sha384,
    /// SHA-512 (64-byte digest).  Fast on 64-bit hardware.
    Sha512,
    /// MD5 (16-byte digest).  **Not collision-resistant** — use only for
    /// legacy checksums, never for security.
    Md5,
    /// BLAKE3 (32-byte digest).  Extremely fast, parallel-friendly.
    Blake3,
}

impl HashAlgorithm {
    /// All supported algorithms in display order.
    pub const ALL: &[Self] = &[
        Self::Sha256,
        Self::Sha384,
        Self::Sha512,
        Self::Md5,
        Self::Blake3,
    ];

    /// Human-readable label (e.g. `"SHA-256"`).
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Sha256 => "SHA-256",
            Self::Sha384 => "SHA-384",
            Self::Sha512 => "SHA-512",
            Self::Md5 => "MD5",
            Self::Blake3 => "BLAKE3",
        }
    }

    /// Machine-friendly key used in URL params / JS interop.
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha384 => "sha384",
            Self::Sha512 => "sha512",
            Self::Md5 => "md5",
            Self::Blake3 => "blake3",
        }
    }

    /// Parse from a key string (case-insensitive).
    pub fn from_key(s: &str) -> Result<Self, CoreError> {
        match s.to_lowercase().as_str() {
            "sha256" | "sha-256" => Ok(Self::Sha256),
            "sha384" | "sha-384" => Ok(Self::Sha384),
            "sha512" | "sha-512" => Ok(Self::Sha512),
            "md5" => Ok(Self::Md5),
            "blake3" => Ok(Self::Blake3),
            other => Err(CoreError::UnsupportedAlgorithm(other.to_owned())),
        }
    }

    /// Digest length in bytes.
    pub const fn output_len(&self) -> usize {
        match self {
            Self::Sha256 => 32,
            Self::Sha384 => 48,
            Self::Sha512 => 64,
            Self::Md5 => 16,
            Self::Blake3 => 32,
        }
    }
}

/// One-shot hash: feeds `input` into the chosen algorithm and returns a
/// lowercase hex-encoded digest.
///
/// # Example
///
/// ```
/// use hashbox_core::hash::{hash, HashAlgorithm};
/// let hex = hash(HashAlgorithm::Sha256, b"hello");
/// assert_eq!(hex.len(), 64);
/// ```
pub fn hash(algorithm: HashAlgorithm, input: &[u8]) -> String {
    match algorithm {
        HashAlgorithm::Sha256 => {
            use sha2::Digest;
            hex::encode(Sha256::digest(input))
        }
        HashAlgorithm::Sha384 => {
            use sha2::Digest;
            hex::encode(Sha384::digest(input))
        }
        HashAlgorithm::Sha512 => {
            use sha2::Digest;
            hex::encode(Sha512::digest(input))
        }
        HashAlgorithm::Md5 => {
            use md5::Digest;
            hex::encode(md5::Md5::digest(input))
        }
        HashAlgorithm::Blake3 => {
            let out = blake3::hash(input);
            hex::encode(out.as_bytes())
        }
    }
}

// ---------------------------------------------------------------------------
// Streaming / incremental hashing
// ---------------------------------------------------------------------------

/// An incremental hasher that implements [`Write`].
///
/// Created via [`hash_streaming`].  After feeding all bytes, call
/// [`finalize_hex`](HashWriter::finalize_hex) to obtain the hex digest.
///
/// # Example
///
/// ```
/// use hashbox_core::hash::{hash_streaming, HashAlgorithm};
/// use std::io::Write;
///
/// let mut w = hash_streaming(HashAlgorithm::Sha256);
/// w.write_all(b"hello ").unwrap();
/// w.write_all(b"world").unwrap();
/// let hex = w.finalize_hex();
/// ```
pub enum HashWriter {
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    Md5(md5::Md5),
    Blake3(blake3::Hasher),
}

impl Write for HashWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Sha256(h) => {
                sha2::Digest::update(h, buf);
                Ok(buf.len())
            }
            Self::Sha384(h) => {
                sha2::Digest::update(h, buf);
                Ok(buf.len())
            }
            Self::Sha512(h) => {
                sha2::Digest::update(h, buf);
                Ok(buf.len())
            }
            Self::Md5(h) => {
                md5::Digest::update(h, buf);
                Ok(buf.len())
            }
            Self::Blake3(h) => {
                h.update(buf);
                Ok(buf.len())
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl HashWriter {
    /// Consume the writer and return the final lowercase hex digest.
    pub fn finalize_hex(self) -> String {
        match self {
            Self::Sha256(h) => {
                use sha2::Digest;
                hex::encode(h.finalize())
            }
            Self::Sha384(h) => {
                use sha2::Digest;
                hex::encode(h.finalize())
            }
            Self::Sha512(h) => {
                use sha2::Digest;
                hex::encode(h.finalize())
            }
            Self::Md5(h) => {
                use md5::Digest;
                hex::encode(h.finalize())
            }
            Self::Blake3(h) => hex::encode(h.finalize().as_bytes()),
        }
    }

    /// Return the algorithm this writer was created for.
    pub fn algorithm(&self) -> HashAlgorithm {
        match self {
            Self::Sha256(_) => HashAlgorithm::Sha256,
            Self::Sha384(_) => HashAlgorithm::Sha384,
            Self::Sha512(_) => HashAlgorithm::Sha512,
            Self::Md5(_) => HashAlgorithm::Md5,
            Self::Blake3(_) => HashAlgorithm::Blake3,
        }
    }
}

/// Create a [`HashWriter`] for the given algorithm.
///
/// Use this for incremental / streaming hashing — feed bytes via
/// [`Write::write`], then call [`HashWriter::finalize_hex`].
pub fn hash_streaming(algorithm: HashAlgorithm) -> HashWriter {
    match algorithm {
        HashAlgorithm::Sha256 => HashWriter::Sha256(Sha256::new()),
        HashAlgorithm::Sha384 => HashWriter::Sha384(Sha384::new()),
        HashAlgorithm::Sha512 => HashWriter::Sha512(Sha512::new()),
        HashAlgorithm::Md5 => HashWriter::Md5(md5::Md5::new()),
        HashAlgorithm::Blake3 => HashWriter::Blake3(blake3::Hasher::new()),
    }
}

// ---------------------------------------------------------------------------
// HMAC
// ---------------------------------------------------------------------------

/// Compute HMAC-SHA-256 and return a lowercase hex digest.
///
/// # Errors
///
/// Returns [`CoreError::HmacInvalidKey`] if `key` is empty.
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Result<String, CoreError> {
    if key.is_empty() {
        return Err(CoreError::HmacInvalidKey);
    }
    use hmac::{Hmac, Mac};
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take any key length");
    mac.update(message);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Compute HMAC-SHA-512 and return a lowercase hex digest.
///
/// # Errors
///
/// Returns [`CoreError::HmacInvalidKey`] if `key` is empty.
pub fn hmac_sha512(key: &[u8], message: &[u8]) -> Result<String, CoreError> {
    if key.is_empty() {
        return Err(CoreError::HmacInvalidKey);
    }
    use hmac::{Hmac, Mac};
    let mut mac = Hmac::<Sha512>::new_from_slice(key).expect("HMAC can take any key length");
    mac.update(message);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_answer() {
        let h = hash(HashAlgorithm::Sha256, b"abc");
        assert_eq!(
            h,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn md5_known_answer() {
        let h = hash(HashAlgorithm::Md5, b"hello");
        assert_eq!(h, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn blake3_known_answer() {
        let h = hash(HashAlgorithm::Blake3, b"foo");
        assert_eq!(
            h,
            "04e0bb39f30b1a3feb89f536c93be15055480df748674b90e4e8e8e5c910922b"
        );
    }

    #[test]
    fn streaming_equivalent() {
        use std::io::Write;
        let data = b"the quick brown fox jumps over the lazy dog";
        let one_shot = hash(HashAlgorithm::Sha256, data);

        let mut w = hash_streaming(HashAlgorithm::Sha256);
        w.write_all(data).unwrap();
        assert_eq!(w.finalize_hex(), one_shot);
    }

    #[test]
    fn hmac_known_answer() {
        let h = hmac_sha256(b"key", b"The quick brown fox").unwrap();
        assert_eq!(
            h,
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
        );
    }

    #[test]
    fn hmac_empty_key_rejected() {
        assert!(hmac_sha256(b"", b"msg").is_err());
        assert!(hmac_sha512(b"", b"msg").is_err());
    }

    #[test]
    fn all_algorithms_produce_expected_lengths() {
        let data = b"hashbox test vector";
        for alg in HashAlgorithm::ALL {
            let h = hash(*alg, data);
            assert_eq!(h.len(), alg.output_len() * 2);
        }
    }

    #[test]
    fn from_key_roundtrip() {
        for alg in HashAlgorithm::ALL {
            let parsed = HashAlgorithm::from_key(alg.key()).unwrap();
            assert_eq!(*alg, parsed);
        }
    }

    #[test]
    fn from_key_case_insensitive() {
        assert_eq!(
            HashAlgorithm::from_key("SHA256").unwrap(),
            HashAlgorithm::Sha256
        );
        assert_eq!(
            HashAlgorithm::from_key("Sha-256").unwrap(),
            HashAlgorithm::Sha256
        );
    }
}
