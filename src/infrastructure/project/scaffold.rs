use crate::domain::project::manifest;
use crate::infra::base64_decoder;
use crate::infra::file_system;
use crate::infra::verification_runtime::docker::container::TARGET_RVT_DIRECTORY;
use anyhow::Result;
use color_eyre::Report;
use std::{env, fs, fs::File, io::prelude::*, path};

fn scaffold_source_directory(project_id: &str) -> Result<String, Report> {
    let project_directory = format_directory_path_to_scaffold(project_id);
    let source_directory =
        [project_directory, "src".to_string()].join(path::MAIN_SEPARATOR.to_string().as_str());
    file_system::ensure_directory_exists(&source_directory)?;

    Ok(source_directory)
}

fn find_source_by_hash(project_id: &str) -> Result<String, Report> {
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

fn scaffold_entry_point(project_id: &str) -> Result<(), Report> {
    let project_source_directory = scaffold_source_directory(project_id)?;
    let entry_point = [project_source_directory.as_str(), "main.rs"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let source = find_source_by_hash(project_id)?;
    let decoded_file_contents = base64_decoder::decode(source).unwrap();

    let mut file = File::create(entry_point)?;
    file.write_all(decoded_file_contents.as_bytes())?;

    Ok(())
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
/// use safepkt_server::infra::project_scaffold;
///
/// let project_id = "0_invalid_package_name_starting_with_a_number";
/// let project_name = project_scaffold::format_project_name(project_id);
/// assert!(project_name.chars().next().unwrap().is_alphabetic());
/// ```
///
pub fn format_project_name(project_id: &str) -> String {
    format!("{}{}", "safepkt_", project_id)
}

fn scaffold_manifest(project_id: &str) -> Result<(), Report> {
    let prefixed_project_id = format_project_name(project_id);
    let manifest_contents =
        manifest::make_manifest(prefixed_project_id.as_str(), TARGET_RVT_DIRECTORY);
    let manifest_path = [env::temp_dir().to_str().unwrap(), project_id, "Cargo.toml"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut manifest_file = File::create(manifest_path)?;
    manifest_file.write_all(manifest_contents.as_bytes())?;

    Ok(())
}

/// Format the path to a directory  
/// to be scaffolded.
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::project_scaffold;
///
/// let directory_name = "project_dir";
///
/// let actual_path = project_scaffold::format_directory_path_to_scaffold(directory_name);
/// assert_eq!(format!("/tmp/{}", directory_name), actual_path);
/// ```
///
pub fn format_directory_path_to_scaffold(project_id: &str) -> String {
    [env::temp_dir().to_str().unwrap(), project_id].join(path::MAIN_SEPARATOR.to_string().as_str())
}

pub async fn scaffold_project(project_id: &str) -> Result<(), Report> {
    scaffold_entry_point(project_id)?;
    scaffold_manifest(project_id)?;

    Ok(())
}
