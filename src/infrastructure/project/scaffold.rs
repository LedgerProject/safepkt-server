use crate::domain::project::manifest;
use crate::infra::base64_decoder;
use crate::infra::file_system;
use crate::infra::verification_runtime::docker::container::TARGET_RVT_DIRECTORY;
use anyhow::Result;
use color_eyre::Report;
use fungus::prelude::*;
use std::{env, fs, fs::File, path};

/// Create a project source ("./src") directory.
fn create_project_source_directory(project_id: &str) -> Result<String, Report> {
    let project_directory = format_directory_path_to_scaffold(project_id);
    let source_directory_path =
        [project_directory, "src".to_string()].join(path::MAIN_SEPARATOR.to_string().as_str());
    file_system::ensure_directory_exists(&source_directory_path)?;

    Ok(source_directory_path)
}

#[test]
fn it_creates_a_project_source_directory() {
    use std::env;
    use std::fs;
    use std::path::Path;

    env::set_var("SOURCE_DIRECTORY", "/tmp");

    let project_id = "project_id";
    let actual_source_directory_path = create_project_source_directory(project_id).unwrap();

    let expected_scaffolded_project_source_directory = "/tmp/project_id/src";
    assert_eq!(
        expected_scaffolded_project_source_directory,
        actual_source_directory_path
    );
    assert!(Path::new(actual_source_directory_path.as_str()).exists());

    assert!(fs::remove_dir_all("/tmp/project_id").is_ok());
}

/// Find a source by project id in the file system.  
/// The project id is a truncated hash of the source file content.
fn find_source_by_project_id(project_id: &str) -> Result<String, Report> {
    let uploaded_source_directory = file_system::get_uploaded_source_directory()?;
    let source_path = [
        uploaded_source_directory.as_str(),
        format!(
            "{}{}",
            project_id,
            file_system::BASE64_ENCODED_SOURCE_EXTENSION
        )
        .as_str(),
    ]
    .join(path::MAIN_SEPARATOR.to_string().as_str());
    file_system::guard_against_missing_source(&source_path)?;

    Ok(fs::read_to_string(source_path)?)
}

#[test]
fn it_finds_a_source_in_the_file_system() {
    use crate::infra::scaffold;
    use std::env;
    use std::fs;
    use std::io::Write;

    let expected_source_code = "Zm4gbWFpbigpIHt9";

    let mut file = fs::File::create("/tmp/project_id.rs.b64").unwrap();
    assert!(file.write_all(expected_source_code.as_bytes()).is_ok());

    env::set_var("SOURCE_DIRECTORY", "/tmp");

    let actual_source = scaffold::find_source_by_project_id("project_id").unwrap();
    let actual_source = actual_source.as_str();

    assert_eq!(expected_source_code, actual_source);

    assert!(fs::remove_file("/tmp/project_id.rs.b64").is_ok());
}

/// Create a project source directory and its parents if needed,
/// before creating the project entry point (main.rs),
/// which contains the source of a project found by project id.
fn create_entry_point(project_id: &str) -> Result<(), Report> {
    let project_source_directory = create_project_source_directory(project_id)?;
    let entry_point = [project_source_directory.as_str(), "main.rs"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let source = find_source_by_project_id(project_id)?;
    let decoded_file_contents = base64_decoder::decode(source).unwrap();

    let mut file = File::create(entry_point)?;
    file.write_all(decoded_file_contents.as_bytes())?;

    Ok(())
}

/// Create a project source directory and its parents if needed,
/// before creating the project library (lib.rs),
/// which contains the source of a library recoverable by project id.
fn create_library(project_id: &str) -> Result<(), Report> {
    let project_source_directory = create_project_source_directory(project_id)?;
    let entry_point = [project_source_directory.as_str(), "lib.rs"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let source = find_source_by_project_id(project_id)?;
    let decoded_file_contents = base64_decoder::decode(source).unwrap();

    let mut file = File::create(entry_point)?;
    file.write_all(decoded_file_contents.as_bytes())?;

    let project_directory = format_directory_path_to_scaffold(project_id);
    let project = path::Path::new(project_directory.as_str());
    sys::chmod(project, 0o777).expect("Can not change source directory permissions.");

    Ok(())
}

#[test]
fn it_creates_an_entry_point() {
    use crate::test;
    use std::env;
    use std::fs;
    use std::path;

    env::set_var("SOURCE_DIRECTORY", "/tmp");

    let random_prefix = test::generate_random_letters();
    let project_id = format!("{}_my_project_id", random_prefix.as_str());

    let source_path = format!("/tmp/{}_my_project_id.rs.b64", random_prefix.as_str());

    let mut file = File::create(source_path).unwrap();
    assert!(file.write_all("Zm4gbWFpbigpIHt9".as_bytes()).is_ok());

    let expected_entry_point_path =
        format!("/tmp/{}_my_project_id/src/main.rs", random_prefix.as_str());

    assert!(create_entry_point(project_id.as_str()).is_ok());
    assert!(path::Path::new(expected_entry_point_path.as_str()).exists());
    assert_eq!(
        "fn main() {}",
        fs::read_to_string(expected_entry_point_path.as_str()).unwrap()
    );

    assert!(fs::remove_dir_all(format!("/tmp/{}_my_project_id", random_prefix.as_str())).is_ok());
    assert!(fs::remove_file(format!(
        "/tmp/{}_my_project_id.rs.b64",
        random_prefix.as_str()
    ))
    .is_ok());
}

/// Format a project name from a project id  
/// for inclusion in a manifest.
///
/// It should start with a letter  
/// so that is a valid package name.
///
/// # Examples
///
/// ```
/// use safepkt_backend::infra::scaffold;
///
/// let project_id = "0_invalid_package_name_starting_with_a_number";
/// let project_name = scaffold::format_project_name(project_id);
/// assert!(project_name.chars().next().unwrap().is_alphabetic());
/// ```
///
pub fn format_project_name(project_id: &str) -> String {
    format!("{}{}", "safepkt_", project_id)
}

/// Create a manifest at the root of a scaffolded project
fn create_manifest(project_id: &str) -> Result<(), Report> {
    let prefixed_project_id = format_project_name(project_id);
    let manifest_contents =
        manifest::make_manifest(prefixed_project_id.as_str(), TARGET_RVT_DIRECTORY);
    let manifest_path = [env::temp_dir().to_str().unwrap(), project_id, "Cargo.toml"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut manifest_file = File::create(manifest_path)?;
    manifest_file.write_all(manifest_contents.as_bytes())?;

    Ok(())
}

#[test]
fn it_creates_a_project_manifest() {
    use std::fs;
    use std::path::Path;

    if fs::create_dir_all("/tmp/my_project_id").is_ok() {
        assert!(create_manifest("my_project_id").is_ok());
    } else {
        panic!("Could not create project directory before creating its manifest.");
    }

    let expected_manifest_path = "/tmp/my_project_id/Cargo.toml";
    assert!(Path::new(expected_manifest_path).exists());

    let manifest_content = fs::read_to_string(expected_manifest_path).unwrap();
    assert!(manifest_content.contains("safepkt_my_project_id"));

    assert!(fs::remove_dir_all("/tmp/my_project_id").is_ok());
}

/// Format the path to a directory  
/// to be scaffolded.
///
/// # Examples
///
/// ```
/// use safepkt_backend::infra::scaffold;
///
/// let actual_path = scaffold::format_directory_path_to_scaffold("project_dir");
/// assert_eq!("/tmp/project_dir", actual_path);
/// ```
///
pub fn format_directory_path_to_scaffold(project_id: &str) -> String {
    [env::temp_dir().to_str().unwrap(), project_id].join(path::MAIN_SEPARATOR.to_string().as_str())
}

/// Scaffold a project from a source file,  
/// which has been uploaded before by:
///  - creating the root directory
///  - creating the source directory (./src)
///  - creating the entry point (./src/main.rs)
///  - creating the manifest (./Cargo.toml)
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use std::fs;
/// use std::io::Write;
/// use std::env;
/// use safepkt_backend::infra::scaffold;
///
/// env::set_var("SOURCE_DIRECTORY", "/tmp");
///
/// let mut file = fs::File::create("/tmp/my_project_id.rs.b64").unwrap();
/// assert!(file.write_all("Zm4gbWFpbigpIHt9".as_bytes()).is_ok());
///
/// assert!(scaffold::scaffold_project("my_project_id").is_ok());
///
/// assert!(Path::new("/tmp/my_project_id/src/main.rs").exists());
///
/// let actual_entry_point_contents = fs::read_to_string(Path::new("/tmp/my_project_id/src/main.rs")).unwrap();
/// assert_eq!("fn main() {}", actual_entry_point_contents.as_str());
///
/// assert!(Path::new("/tmp/my_project_id/Cargo.toml").exists());
///
/// assert!(fs::remove_dir_all("/tmp/my_project_id").is_ok());
/// ```
///
pub fn scaffold_project(project_id: &str) -> Result<(), Report> {
    if create_entry_point(project_id).is_ok() {
        create_manifest(project_id)?;
    }

    Ok(())
}

pub fn scaffold_library(project_id: &str) -> Result<(), Report> {
    if create_library(project_id).is_ok() {
        create_manifest(project_id)?;
    }

    Ok(())
}
