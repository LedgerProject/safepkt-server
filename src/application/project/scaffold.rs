use crate::application::project::manifest::get_manifest;
use crate::infrastructure::docker::client::run_detached_static_analysis;
use crate::infrastructure::service::decoder::base64_decode;
use crate::infrastructure::service::file_system::*;
use anyhow::Result;
use color_eyre::Report;
use std::{env, fs, fs::File, io::prelude::*, path};

fn scaffold_source_directory(source_hash: &str) -> Result<String, Report> {
    let project_directory = get_scaffolded_project_directory(source_hash);
    let source_directory =
        [project_directory, "src".to_string()].join(path::MAIN_SEPARATOR.to_string().as_str());
    get_path_or_create(&source_directory)?;

    Ok(source_directory)
}

fn find_source_by_hash(source_hash: &str) -> Result<String, Report> {
    let uploaded_source_directory = get_uploaded_source_directory()?;
    let source_path = [
        uploaded_source_directory.as_str(),
        format!("{}{}", source_hash, BASE64_ENCODED_SOURCE_EXTENSION).as_str(),
    ]
    .join(path::MAIN_SEPARATOR.to_string().as_str());
    get_path_or_err(&source_path).unwrap();

    Ok(fs::read_to_string(source_path)?)
}

fn scaffold_entry_point(source_hash: &str) -> Result<(), Report> {
    let project_source_directory = scaffold_source_directory(source_hash)?;
    let entry_point = [project_source_directory.as_str(), "main.rs"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let source = find_source_by_hash(source_hash)?;
    let decoded_file_contents = base64_decode(source).unwrap();

    let mut file = File::create(entry_point)?;
    file.write_all(decoded_file_contents.as_bytes())?;

    Ok(())
}

fn scaffold_manifest(source_hash: &str) -> Result<(), Report> {
    let manifest_contents = get_manifest(source_hash);
    let manifest_path = [env::temp_dir().to_str().unwrap(), source_hash, "Cargo.toml"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut manifest_file = File::create(manifest_path)?;
    manifest_file.write_all(manifest_contents.as_bytes())?;

    Ok(())
}

pub fn get_scaffolded_project_directory(source_hash: &str) -> String {
    [env::temp_dir().to_str().unwrap(), source_hash].join(path::MAIN_SEPARATOR.to_string().as_str())
}

pub async fn scaffold_project(source_hash: &str) -> Result<(), Report> {
    scaffold_entry_point(source_hash)?;
    scaffold_manifest(source_hash)?;
    run_detached_static_analysis(source_hash).await?;

    Ok(())
}
