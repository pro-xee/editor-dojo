# Result Integrity System

## Overview

The editor-dojo integrity system uses cryptographic signing to prevent tampering with locally stored challenge results and recordings. This helps maintain the legitimacy of achievements and progress tracking while keeping the codebase open-source.

## How It Works

### Signature Generation

When you complete a challenge with a recording, the system:

1. **Calculates a SHA256 hash** of the asciinema recording file (`.cast` file)
2. **Creates a signature** using HMAC-SHA256 over:
   - Challenge ID
   - Keystroke count
   - Completion time (milliseconds)
   - Timestamp (RFC3339 format)
   - Recording file hash
3. **Stores the signature** alongside the result in `progress.json`

### Signature Verification

When results are loaded, the system can verify:

1. **Signature integrity** - Ensures the result data hasn't been modified
2. **Recording integrity** - Verifies the recording file matches its stored hash

### Build Types

#### Development Builds (Default)

- Use a public fallback key hardcoded in `build.rs`
- Display warning that results are not secured
- Perfect for local development and testing
- Anyone can build and use these locally

#### Production/Release Builds

- Built via GitHub Actions with a secret signing key from repository secrets
- Key is injected at build time via `SIGNING_KEY` environment variable
- Official releases are signed with the production key
- Results from official builds can be verified against tampering

## Data Structure

Results in `progress.json` include these integrity fields (all optional for backwards compatibility):

```json
{
  "challenges": {
    "challenge-id": {
      "completed": true,
      "best_time_secs": 10,
      "best_keystrokes": 15,
      "first_completed_at": "2025-01-15T10:30:00Z",
      "recording_hash": "abc123...",
      "signature": "def456...",
      "signature_version": 1
    }
  }
}
```

## Verification Status

Each result has one of these verification statuses (computed at runtime):

- **Legacy** - Old result without signature (backwards compatible)
- **Unverified** - Has signature but not yet checked
- **Verified** ✅ - Signature and recording hash both valid
- **SignatureFailed** ❌ - Result data has been tampered with
- **RecordingHashFailed** ❌ - Recording file has been modified

## Security Limitations

**IMPORTANT**: This is a **tamper detection** system, not tamper prevention. Be aware of these limitations:

### What This System DOES

✅ Detects if result JSON files have been manually edited
✅ Detects if recording files have been modified
✅ Prevents casual cheating by editing save files
✅ Supports backwards compatibility with unsigned legacy data

### What This System DOES NOT Do

❌ Prevent a determined attacker from extracting the signing key from the binary
❌ Prevent users from building their own version with a different key
❌ Prevent binary modification or memory inspection
❌ Provide server-side validation (no remote verification)
❌ Use advanced obfuscation or anti-debugging techniques

### Why These Limitations Exist

This is an educational tool for personal skill development, not a high-security competitive platform. The integrity system is designed to:

1. **Discourage casual cheating** - Makes it non-trivial to fake results
2. **Maintain open-source nature** - All code is publicly visible
3. **Enable local development** - Developers can build and test locally
4. **Support community trust** - Official releases have verifiable signatures

**Bottom line**: A sufficiently motivated person could reverse engineer the signing key from a release binary and create fake results. This is acceptable because the tool is primarily for personal development and self-improvement.

## For Users

### Checking Your Build Type

When you build the project, you'll see one of these warnings:

```
warning: Building with DEVELOPMENT signing key (INSECURE)
```

or

```
warning: Building with PRODUCTION signing key
```

### Using Official Releases

Official releases from GitHub are built with the production signing key. Your results from official builds will have verifiable signatures.

### Building Locally

Local builds use the development key. This is fine for personal use! The integrity system will still work to prevent accidental file corruption.

## For Developers

### Building Locally

```bash
# Development build (uses public fallback key)
cargo build

# You'll see: "Building with DEVELOPMENT signing key (INSECURE)"
```

### Testing the Integrity System

```bash
# Build and run
cargo run

# Complete a challenge with recording
# Results will be signed with development key

# View progress file
cat ~/.local/share/editor-dojo/progress.json

# You should see signature and recording_hash fields
```

### Key Rotation

The system supports multiple signature versions for future key rotation:

1. Update `SIGNATURE_VERSION` constant in `src/infrastructure/crypto.rs`
2. Update the signing key in GitHub Secrets
3. Old results signed with previous versions remain valid
4. Verification logic checks the `signature_version` field

## For Maintainers

### Setting Up Production Builds

1. **Generate a signing key** (256-bit hex string):
   ```bash
   openssl rand -hex 32
   ```

2. **Add to GitHub Secrets**:
   - Go to repository Settings → Secrets and variables → Actions
   - Create new secret named `SIGNING_KEY`
   - Paste the 64-character hex string
   - **CRITICAL**: Never commit this key to the repository!

3. **Verify GitHub Actions workflow**:
   - The workflow at `.github/workflows/release.yml` injects the key
   - Builds are triggered on version tags (e.g., `v1.0.0`)
   - Multiple platforms are built with the same key

### Rotating the Signing Key

If the production key is compromised:

1. Generate a new signing key
2. Increment `SIGNATURE_VERSION` in code
3. Update `SIGNING_KEY` secret in GitHub
4. Update verification logic to accept both old and new versions
5. Document the rotation in release notes

### Verifying Build Security

The build system ensures the key is never exposed:

- ✅ Key is injected only during build via environment variable
- ✅ Key is obfuscated with XOR encoding in the binary
- ✅ Key never appears in git history or source code
- ✅ Build logs don't print the key value
- ✅ Temporary files are cleaned up after build

## Architecture

### Build Script (`build.rs`)

- Reads `SIGNING_KEY` environment variable (production) or uses fallback (dev)
- XOR-obfuscates the key with a constant
- Writes obfuscated key to `OUT_DIR/signing_key.bin`
- Generates constants for key length and obfuscation key
- Creates build mode indicator (production vs development)

### Crypto Module (`src/infrastructure/crypto.rs`)

- Loads and deobfuscates signing key at runtime
- Provides `sign_result()` function for HMAC-SHA256 signing
- Provides `verify_signature()` with constant-time comparison
- Provides `calculate_file_hash()` for SHA256 hashing
- Provides `verify_recording_hash()` for file integrity checking

### Progress Tracker (`src/application/progress_tracker.rs`)

- Generates signatures when recording solutions
- Includes recording hash in signature input
- Stores signature and hash with challenge stats
- Provides verification methods for loaded results

### Domain Model (`src/domain/challenge_stats.rs`)

- `ChallengeStats` stores optional integrity fields
- `VerificationStatus` enum tracks verification state
- Backwards compatible with legacy unsigned results

## Backwards Compatibility

The system is fully backwards compatible:

- Old results without signatures still work (status: `Legacy`)
- New results get signed automatically if a recording exists
- Optional fields in JSON (`#[serde(default)]`)
- Migration is seamless - no user action required

## Future Enhancements

Potential improvements (not in current scope):

- **Server-side validation** - Remote verification of results
- **Leaderboards** - Compare verified results with others
- **Achievement NFTs** - Blockchain-based achievement proof
- **Code obfuscation** - Make key extraction harder (diminishing returns)
- **Hardware key storage** - Use OS keychain for key storage

## Questions?

See the main [README.md](README.md) for general usage documentation.

For security concerns or questions about the integrity system, please open an issue on GitHub.
