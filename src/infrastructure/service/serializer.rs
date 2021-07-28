use crate::domain::entity::source::Source;
use anyhow::Result;

pub fn deserialize<'a>(subject: &'a str) -> Result<Source<'a>> {
    let source: Source = serde_json::from_str(subject)?;

    Ok(source)
}