use crate::domain::entity::source::Source;
use anyhow::Result;

pub fn deserialize(subject: &str) -> Result<Source> {
    let source: Source = serde_json::from_str(subject)?;

    Ok(source)
}
