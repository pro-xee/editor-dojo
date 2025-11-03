use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

type HmacSha256 = Hmac<Sha256>;

/// Current signature version for key rotation support
pub const SIGNATURE_VERSION: u32 = 1;

// Include build-time generated constants
include!(concat!(env!("OUT_DIR"), "/key_constants.rs"));
include!(concat!(env!("OUT_DIR"), "/build_mode.rs"));

/// Load and deobfuscate the signing key
fn get_signing_key() -> Vec<u8> {
    // Load obfuscated key from build output
    let key_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/signing_key.bin"));

    // Deobfuscate using XOR
    key_bytes.iter().map(|&b| b ^ OBFUSCATION_KEY).collect()
}

/// Calculate SHA256 hash of a file
pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let contents = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Sign result data using HMAC-SHA256
///
/// Creates a signature over the concatenated data:
/// challenge_id|strokes|time_ms|timestamp|recording_hash
pub fn sign_result(
    challenge_id: &str,
    strokes: u32,
    time_ms: u64,
    timestamp: &str,
    recording_hash: &str,
) -> String {
    let key = get_signing_key();
    let mut mac = HmacSha256::new_from_slice(&key)
        .expect("HMAC can take key of any size");

    // Create canonical representation for signing
    let data = format!(
        "{}|{}|{}|{}|{}",
        challenge_id, strokes, time_ms, timestamp, recording_hash
    );

    mac.update(data.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Verify result signature using constant-time comparison
///
/// Returns true if signature is valid, false otherwise
pub(crate) fn verify_signature(
    challenge_id: &str,
    strokes: u32,
    time_ms: u64,
    timestamp: &str,
    recording_hash: &str,
    signature: &str,
    _signature_version: u32, // For future key rotation
) -> bool {
    // Re-compute expected signature
    let expected = sign_result(challenge_id, strokes, time_ms, timestamp, recording_hash);

    // Constant-time comparison to prevent timing attacks
    constant_time_compare(&expected, signature)
}

/// Verify that recording file hash matches stored hash
pub(crate) fn verify_recording_hash<P: AsRef<Path>>(
    recording_path: P,
    expected_hash: &str,
) -> anyhow::Result<bool> {
    if !recording_path.as_ref().exists() {
        return Ok(false);
    }

    let actual_hash = calculate_file_hash(recording_path)?;
    Ok(constant_time_compare(&actual_hash, expected_hash))
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut diff = 0u8;
    for i in 0..a_bytes.len() {
        diff |= a_bytes[i] ^ b_bytes[i];
    }

    diff == 0
}

/// Check if this is a production build with secure signing key
pub(crate) fn is_production_build() -> bool {
    IS_PRODUCTION_BUILD
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_sign_and_verify() {
        let signature = sign_result(
            "test-challenge-1",
            42,
            10500,
            "2025-01-15T10:30:00Z",
            "abc123def456",
        );

        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64); // SHA256 produces 32 bytes = 64 hex chars

        // Verify with correct data
        assert!(verify_signature(
            "test-challenge-1",
            42,
            10500,
            "2025-01-15T10:30:00Z",
            "abc123def456",
            &signature,
            1,
        ));

        // Verify fails with incorrect data
        assert!(!verify_signature(
            "test-challenge-1",
            43, // Different keystroke count
            10500,
            "2025-01-15T10:30:00Z",
            "abc123def456",
            &signature,
            1,
        ));

        assert!(!verify_signature(
            "test-challenge-2", // Different challenge
            42,
            10500,
            "2025-01-15T10:30:00Z",
            "abc123def456",
            &signature,
            1,
        ));
    }

    #[test]
    fn test_file_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test content").unwrap();

        let hash = calculate_file_hash(temp_file.path()).unwrap();
        assert_eq!(hash.len(), 64); // SHA256 = 64 hex chars

        // Same content should produce same hash
        let hash2 = calculate_file_hash(temp_file.path()).unwrap();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_verify_recording_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "recording data").unwrap();

        let hash = calculate_file_hash(temp_file.path()).unwrap();

        // Correct hash should verify
        assert!(verify_recording_hash(temp_file.path(), &hash).unwrap());

        // Wrong hash should fail
        assert!(!verify_recording_hash(temp_file.path(), "wrong_hash").unwrap());
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare("abc123", "abc123"));
        assert!(!constant_time_compare("abc123", "abc124"));
        assert!(!constant_time_compare("abc123", "abc12"));
        assert!(!constant_time_compare("abc", "abc123"));
    }

    #[test]
    fn test_signature_deterministic() {
        let sig1 = sign_result("test", 10, 5000, "2025-01-01T00:00:00Z", "hash");
        let sig2 = sign_result("test", 10, 5000, "2025-01-01T00:00:00Z", "hash");
        assert_eq!(sig1, sig2);
    }
}
