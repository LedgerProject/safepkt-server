pub fn get_manifest(source_hash: &str) -> String {
    let template = r#"
[package]
name = "{{ hash }}"
version = "0.1.0"
authors = [""]
edition = "2018"

[[bin]]
name = "{{ hash }}"
path = "src/main.rs"

[dependencies]
verification-annotations = { path="/home/rust-verification-tools/verification-annotations" }

[features]
verifier-klee = ["verification-annotations/verifier-klee"]

[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path="/home/rust-verification-tools/propverify" }
"#;

    template.replace("{{ hash }}", source_hash)
}
