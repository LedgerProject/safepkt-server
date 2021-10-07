/// Make a manifest from a package name and  
/// a path to Rust Verification Tools
///
/// # Examples
///
/// ```
/// use safepkt_backend::app::domain::manifest;
///
/// let package_name = "safepkt_backend";
/// let rust_verification_tools_directory = "/home/rvt";
///
/// let manifest = manifest::make_manifest(package_name, rust_verification_tools_directory);
///
/// assert!(manifest.contains(package_name));
/// assert!(manifest.contains(rust_verification_tools_directory));
/// ```
///
pub fn make_manifest(package_name: &str, rvt_dir_path: &str) -> String {
    let template = r#"
[package]
name = "{{ name }}"
version = "0.1.0"
authors = ["CJDNS SASU"]
edition = "2018"

[[bin]]
name = "{{ name }}"
path = "src/main.rs"

[dependencies]
verification-annotations = { path="{{ rust_verifications_tools }}/verification-annotations" }
ink_primitives = { version = "3.0.0-rc3", default-features = false }
ink_metadata = { version = "3.0.0-rc3", default-features = false, features = ["derive"], optional = true }
ink_env = { version = "3.0.0-rc3", default-features = false }
ink_storage = { version = "3.0.0-rc3", default-features = false }
ink_lang = { version = "3.0.0-rc3", default-features = false }
scale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }
scale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }

[lib]
name = "nft"
path = "lib.rs"
crate-type = [
	# Used for normal contract Wasm blobs.
	"cdylib",
]

[features]
verifier-klee = ["verification-annotations/verifier-klee"]
default = ["std"]
std = [
    "ink_metadata/std",
    "ink_env/std",
    "ink_storage/std",
    "ink_primitives/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []

[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path="{{ rust_verifications_tools }}/propverify" }
"#;

    template
        .replace("{{ name }}", package_name)
        .replace("{{ rust_verifications_tools }}", rvt_dir_path)
}

#[test]
fn it_makes_a_manifest() {
    use crate::domain::project::manifest;

    let expected_manifest = r#"
[package]
name = "test"
version = "0.1.0"
authors = ["CJDNS SASU"]
edition = "2018"

[[bin]]
name = "test"
path = "src/main.rs"

[dependencies]
verification-annotations = { path="/rvt/verification-annotations" }

[features]
verifier-klee = ["verification-annotations/verifier-klee"]

[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path="/rvt/propverify" }
"#;

    assert_eq!(expected_manifest, manifest::make_manifest("test", "/rvt"))
}
