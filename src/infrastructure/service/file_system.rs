use anyhow::Result;
use color_eyre::Report;
use hex;
use sha2::{Digest, Sha256};
use std::{env, fs, fs::File, io::prelude::*, path};
use tracing::error;

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize().to_vec();

    hex::encode(hash).chars().into_iter().take(10).collect()
}

pub fn get_path_or_create(path_as_str: &str) -> Result<&path::Path, Report> {
    let path = path::Path::new(path_as_str);

    if !path.exists() {
        fs::create_dir(path)?;
    }

    Ok(path)
}

pub fn get_path_or_err(path_as_str: &str) -> Result<(), Report> {
    let source_path = path::Path::new(path_as_str);

    if !source_path.exists() {
        let error_message = format!("Can not find source at path \"{}\"", path_as_str);
        error!("{}", error_message);
    }

    Ok(())
}

pub fn get_uploaded_source_directory() -> Result<String, Report> {
    let source_directory = env::var("SOURCE_DIRECTORY")?;
    Ok(source_directory)
}

pub fn save_content_on_file_system(content: &[u8]) -> Result<(), Report> {
    let content_hash: String = hash_content(content);
    let uploaded_source_directory = get_uploaded_source_directory()?;
    let file_name = format!("{}.rs.b64", content_hash);
    let file_path =
        [uploaded_source_directory, file_name].join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut file = File::create(file_path)?;
    file.write_all(content)?;

    Ok(())
}
