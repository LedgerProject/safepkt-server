use crate::domain::value_object::Source;
use anyhow::Result;

/// Parse JSON content passed as a string slice  
/// before deserializing it as a Source value object
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::serializer;
///
/// let encoded_source_code = "Zm4gbWFpbigpIHt9";
/// let json_content = format!("{{\"source\": \"{}\"}}", encoded_source_code);
/// let source = serializer::deserialize(json_content.as_str()).unwrap();
/// assert_eq!(encoded_source_code.as_bytes(), source.source());
/// ```
///
pub fn deserialize(subject: &str) -> Result<Source> {
    let source: Source = serde_json::from_str(subject)?;

    Ok(source)
}
