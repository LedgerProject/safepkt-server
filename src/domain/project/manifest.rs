/// Make a manifest from a package name and  
/// a path to Rust Verification Tools
///
/// # Examples
///
/// ```
/// use safepkt_server::app::domain::manifest;
///
/// let package_name = "safepkt_server";
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

[features]
verifier-klee = ["verification-annotations/verifier-klee"]

[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path={{ rust_verifications_tools }}/propverify" }
"#;

    template
        .replace("{{ name }}", package_name)
        .replace("{{ rust_verifications_tools }}", rvt_dir_path)
}

#[test]
fn it_should_make_a_manifest() {
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
propverify = { path=/rvt/propverify" }
"#;

    assert_eq!(expected_manifest, manifest::make_manifest("test", "/rvt"))
}
