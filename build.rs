use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Determine signing key based on build environment
    let signing_key = if let Ok(key) = env::var("SIGNING_KEY") {
        // Production build: use secret key from environment
        println!("cargo:warning=Building with PRODUCTION signing key");
        key
    } else {
        // Development build: use insecure fallback key
        println!("cargo:warning=Building with DEVELOPMENT signing key (INSECURE)");
        "dev_insecure_key_do_not_use_in_production_0123456789abcdef".to_string()
    };

    // Validate key is hex and appropriate length (at least 32 bytes for security)
    let key_bytes = match hex::decode(&signing_key) {
        Ok(bytes) if bytes.len() >= 32 => bytes,
        Ok(bytes) => {
            // Key is too short, pad or fail for production
            if env::var("SIGNING_KEY").is_ok() {
                panic!("SIGNING_KEY must be at least 32 bytes (64 hex characters)");
            }
            // For dev key, just use the string as-is
            signing_key.as_bytes().to_vec()
        }
        Err(_) => {
            // Not valid hex, use string directly (for dev mode)
            signing_key.as_bytes().to_vec()
        }
    };

    // Simple XOR obfuscation to avoid plain text key in binary
    let obfuscation_key: u8 = 0x5A;
    let obfuscated: Vec<u8> = key_bytes.iter().map(|&b| b ^ obfuscation_key).collect();

    // Write obfuscated key to output file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("signing_key.bin");
    fs::write(&dest_path, &obfuscated).expect("Failed to write signing key");

    // Write the obfuscation key constant
    let key_code = format!(
        "pub const OBFUSCATION_KEY: u8 = 0x{:02X};\npub const KEY_LENGTH: usize = {};\n",
        obfuscation_key,
        obfuscated.len()
    );
    let key_const_path = Path::new(&out_dir).join("key_constants.rs");
    fs::write(&key_const_path, key_code).expect("Failed to write key constants");

    // Write build mode indicator
    let is_production = env::var("SIGNING_KEY").is_ok();
    let build_mode = format!(
        "pub const IS_PRODUCTION_BUILD: bool = {};\n",
        is_production
    );
    let mode_path = Path::new(&out_dir).join("build_mode.rs");
    fs::write(&mode_path, build_mode).expect("Failed to write build mode");

    println!("cargo:rerun-if-env-changed=SIGNING_KEY");
}
