//! Exposes the versions of dependent Rust crates to the API health response.
//!
//! Cargo reruns this build script when a dependent crate's manifest changes.

use std::{env, fs, path::Path};

fn package_version(path: &Path) -> String {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let mut in_package = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
        } else if in_package {
            if let Some(value) = line.strip_prefix("version =") {
                let value = value.trim().trim_matches('"');
                assert!(!value.is_empty(), "empty package version: {}", path.display());
                return value.to_string();
            }
        }
    }
    panic!("missing [package].version in {}", path.display());
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory");
    for (name, crate_name) in [
        ("DOMAIN", "doseark_domain"),
        ("APPLICATION", "doseark_application"),
        ("CONTRACTS", "doseark_contracts"),
        ("DB_POSTGRES", "doseark_db_postgres"),
        ("DB_SQLITE", "doseark_db_sqlite"),
    ] {
        let path = Path::new(&manifest_dir)
            .join("../../crates")
            .join(crate_name)
            .join("Cargo.toml");
        println!("cargo:rerun-if-changed={}", path.display());
        println!("cargo:rustc-env=DOSEARK_{name}_VERSION={}", package_version(&path));
    }

    println!("cargo:rerun-if-env-changed=DOSEARK_CHANNEL");
    let channel = env::var("DOSEARK_CHANNEL").unwrap_or_else(|_| "development".to_string());
    assert!(
        !channel.is_empty() && channel.bytes().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.')),
        "DOSEARK_CHANNEL must contain only letters, digits, dots, dashes or underscores"
    );
    println!("cargo:rustc-env=DOSEARK_BUILD_CHANNEL={channel}");
}

#[cfg(test)]
mod tests {
    use super::package_version;
    use std::path::Path;

    #[test]
    fn reads_package_version_from_manifest() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert_eq!(package_version(&path), env!("CARGO_PKG_VERSION"));
    }
}
