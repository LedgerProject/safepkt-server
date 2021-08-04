use crate::domain::value_object::Source;
use anyhow::Result;

/// Parse JSON content before deserializing it.
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::serializer;
///
/// let json_content = r#"
/// {"source": "Zm4gbWFpbigpIHt9"}
/// "#;
///
/// let source = serializer::deserialize(json_content).unwrap();
///
/// let expected_source = "Zm4gbWFpbigpIHt9";
/// assert_eq!(expected_source.as_bytes(), source.source());
/// ```
///
pub fn deserialize(subject: &str) -> Result<Source> {
    let source: Source = serde_json::from_str(subject)?;

    Ok(source)
}
