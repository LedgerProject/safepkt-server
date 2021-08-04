mod application;
mod domain;
mod infrastructure;

pub mod app {
    use super::application;

    pub use application::http::controller;
    pub use application::http::middleware;
    pub use application::http::router;

    pub mod domain {
        use super::super::domain;

        pub use domain::project::manifest;
        pub use domain::value_object;
        pub use domain::verification_runtime;
    }
}

pub mod infra {
    use super::infrastructure;

    pub use infrastructure::project_scaffold;
    pub use infrastructure::service::*;
    pub use infrastructure::signal_handling;
    pub use infrastructure::verification_runtime;
}

#[cfg(test)]
mod test {
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

    #[test]
    fn it_should_save_content_in_file_system() {
        use crate::infra::file_system;
        use std::path::Path;

        dotenv::from_filename("./.env.test").ok();
        let destination_file_path = "/tmp/9f86d08188.rs.b64";

        assert_eq!(
            destination_file_path,
            file_system::save_content_in_file_system("test".as_bytes()).unwrap()
        );
        assert!(Path::exists(Path::new(destination_file_path)));
    }

    #[test]
    fn it_should_decode_base64_encoded_content() {
        use crate::infra::base64_decoder;

        let decoded_contents = base64_decoder::decode(b"dGVzdA==").unwrap();
        assert_eq!("test", decoded_contents);
    }

    #[test]
    fn it_should_guard_against_missing_file() {
        use crate::infra::file_system;
        use std::fs;
        use std::io::Write;

        let file_path = "/tmp/test";

        let mut file = fs::File::create(file_path).unwrap();
        file.write_all("".as_bytes()).unwrap();

        assert_eq!(
            (),
            file_system::guard_against_missing_source(file_path).unwrap()
        );

        fs::remove_file(file_path).unwrap();
    }
}
