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
authors = ["Parity Technologies <admin@parity.io>", "CJDNS SASU"]
edition = "2018"

[dependencies]
verification-annotations = { path="{{ rust_verification_tools }}/verification-annotations" }
ink_primitives = { version = "2.1.0", path = "../../primitives", default-features = false }
ink_abi = { version = "2.1.0", path = "../../abi", default-features = false, features = ["derive"], optional = true }
ink_core = { version = "2.1.0", path = "../../core", default-features = false }
ink_lang = { version = "2.1.0", path = "../../lang", default-features = false }
ink_prelude = { version = "2.1.0", path = "../../prelude", default-features = false }

scale = { package = "parity-scale-codec", version = "1.2", default-features = false, features = ["derive"] }

[dependencies.type-metadata]
git = "https://github.com/type-metadata/type-metadata.git"
rev = "02eae9f35c40c943b56af5b60616219f2b72b47d"
default-features = false
features = ["derive"]
optional = true

[lib]
name = "{{ name }}"
path = "lib.rs"
crate-type = [
	# Used for normal contract Wasm blobs.
	"cdylib",
	# Used for ABI generation.
	"rlib",
]

[features]
verifier-klee = ["verification-annotations/verifier-klee"]
default = ["test-env"]
std = [
    "ink_abi/std",
    "ink_core/std",
    "ink_primitives/std",
    "ink_prelude/std",
    "scale/std",
    "type-metadata/std",
]
test-env = [
    "std",
    "ink_lang/test-env",
]
ink-generate-abi = [
    "std",
    "ink_abi",
    "type-metadata",
    "ink_core/ink-generate-abi",
    "ink_lang/ink-generate-abi",
]
ink-as-dependency = []

[profile.release]
panic = "abort"
lto = true
opt-level = "z"
overflow-checks = true

[workspace]
members = [
	".ink/abi_gen"
]
exclude = [
	".ink"
]

[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path="{{ rust_verification_tools }}/propverify" }
"#;

    template
        .replace("{{ name }}", package_name)
        .replace("{{ rust_verification_tools }}", rvt_dir_path)
}

#[test]
fn it_makes_a_manifest() {
    use crate::domain::project::manifest;

    let expected_manifest = r#"
[package]
name = "test"
version = "0.1.0"
authors = ["Parity Technologies <admin@parity.io>", "CJDNS SASU"]
edition = "2018"

[dependencies]
verification-annotations = { path="/rvt/verification-annotations" }
ink_primitives = { version = "2.1.0", path = "../../primitives", default-features = false }
ink_abi = { version = "2.1.0", path = "../../abi", default-features = false, features = ["derive"], optional = true }
ink_core = { version = "2.1.0", path = "../../core", default-features = false }
ink_lang = { version = "2.1.0", path = "../../lang", default-features = false }
ink_prelude = { version = "2.1.0", path = "../../prelude", default-features = false }

scale = { package = "parity-scale-codec", version = "1.2", default-features = false, features = ["derive"] }

[dependencies.type-metadata]
git = "https://github.com/type-metadata/type-metadata.git"
rev = "02eae9f35c40c943b56af5b60616219f2b72b47d"
default-features = false
features = ["derive"]
optional = true

[lib]
name = "test"
path = "lib.rs"
crate-type = [
	# Used for normal contract Wasm blobs.
	"cdylib",
	# Used for ABI generation.
	"rlib",
]

[features]
verifier-klee = ["verification-annotations/verifier-klee"]
default = ["test-env"]
std = [
    "ink_abi/std",
    "ink_core/std",
    "ink_primitives/std",
    "ink_prelude/std",
    "scale/std",
    "type-metadata/std",
]
test-env = [
    "std",
    "ink_lang/test-env",
]
ink-generate-abi = [
    "std",
    "ink_abi",
    "type-metadata",
    "ink_core/ink-generate-abi",
    "ink_lang/ink-generate-abi",
]
ink-as-dependency = []

[profile.release]
panic = "abort"
lto = true
opt-level = "z"
overflow-checks = true

[workspace]
members = [
	".ink/abi_gen"
]
exclude = [
	".ink"
]

[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path="/rvt/propverify" }
"#;

    assert_eq!(expected_manifest, manifest::make_manifest("test", "/rvt"))
}
