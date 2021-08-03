use crate::domain::project::manifest::get_manifest;
use crate::infrastructure as infra;
use anyhow::Result;
use color_eyre::Report;
use infra::service::decoder::base64_decode;
use infra::service::file_system::*;
use std::{env, fs, fs::File, io::prelude::*, path};

fn scaffold_source_directory(target_hash: &str) -> Result<String, Report> {
    let project_directory = get_scaffolded_project_directory(target_hash);
    let source_directory =
        [project_directory, "src".to_string()].join(path::MAIN_SEPARATOR.to_string().as_str());
    get_path_or_create(&source_directory)?;

    Ok(source_directory)
}

fn find_source_by_hash(target_hash: &str) -> Result<String, Report> {
    let uploaded_source_directory = get_uploaded_source_directory()?;
    let source_path = [
        uploaded_source_directory.as_str(),
        format!("{}{}", target_hash, BASE64_ENCODED_SOURCE_EXTENSION).as_str(),
    ]
    .join(path::MAIN_SEPARATOR.to_string().as_str());
    get_path_or_err(&source_path).unwrap();

    Ok(fs::read_to_string(source_path)?)
}

fn scaffold_entry_point(target_hash: &str) -> Result<(), Report> {
    let project_source_directory = scaffold_source_directory(target_hash)?;
    let entry_point = [project_source_directory.as_str(), "main.rs"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let source = find_source_by_hash(target_hash)?;
    let decoded_file_contents = base64_decode(source).unwrap();

    let mut file = File::create(entry_point)?;
    file.write_all(decoded_file_contents.as_bytes())?;

    Ok(())
}

pub fn prefix_hash(hash: &str) -> String {
    format!("{}{}", "safepkt_", hash)
}

fn scaffold_manifest(target_hash: &str) -> Result<(), Report> {
    let prefixed_target_hash = prefix_hash(target_hash);
    let manifest_contents = get_manifest(prefixed_target_hash.as_str());
    let manifest_path = [env::temp_dir().to_str().unwrap(), target_hash, "Cargo.toml"]
        .join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut manifest_file = File::create(manifest_path)?;
    manifest_file.write_all(manifest_contents.as_bytes())?;

    Ok(())
}

pub fn get_scaffolded_project_directory(target_hash: &str) -> String {
    [env::temp_dir().to_str().unwrap(), target_hash].join(path::MAIN_SEPARATOR.to_string().as_str())
}

pub async fn scaffold_project(target_hash: &str) -> Result<(), Report> {
    scaffold_entry_point(target_hash)?;
    scaffold_manifest(target_hash)?;

    Ok(())
}
