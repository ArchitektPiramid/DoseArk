# doseark_runtime_config

File-backed API profiles and Rust-only credentials resolution. Profile JSON contains a
`credentialsRef` lookup key; passwords and API tokens live in a separate credentials
file and must not be exposed to browser clients. Both profile schema versions 1 and
2 are supported. Storage connectivity and profile-specific rules are checked by the
consuming runtime, not by this crate.

## Checks

From the workspace root:

```powershell
cargo fmt --all -- --check
cargo check -p doseark_runtime_config
cargo test -p doseark_runtime_config
cargo check --workspace --locked
```

Run the unlocked `cargo check` once after a version change and include the updated
`Cargo.lock` in the same commit. Tests use isolated files in the OS temp directory;
they never access your actual profile or credentials files.
