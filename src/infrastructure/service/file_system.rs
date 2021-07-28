use chrono::Utc;
use sha2::{Sha256, Digest};
use std::{env, fs::File, io::prelude::*, path};
use anyhow::Result;
use hex;

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize().to_vec();

    hex::encode(hash)
        .chars()
        .into_iter()
        .take(10)
        .collect()
}

pub fn save_content_on_file_system(content: &[u8]) -> Result<()> {
    let now = Utc::now();
    let today_date_prefix = now.format("%Y-%m-%d");

    let content_hash: String = hash_content(content);

    let source_directory = env::var("SOURCE_DIRECTORY")?;
    let file_name = format!("{}-{}.rs.b64", today_date_prefix, content_hash);
    let file_path = [source_directory, file_name].join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut file = File::create(file_path)?;
    file.write_all(content)?;

    Ok(())
}