use crate::domain::value_object::{Flags, Source};
use anyhow::Result;

/// Parse JSON content before deserializing it into source.
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
/// let source = serializer::deserialize_source(json_content).unwrap();
///
/// let expected_source = "Zm4gbWFpbigpIHt9";
/// assert_eq!(expected_source.as_bytes(), source.source());
/// ```
///
pub fn deserialize_source(subject: &str) -> Result<Source> {
    let source: Source = serde_json::from_str(subject)?;

    Ok(source)
}

/// Parse JSON content before deserializing it into flags.
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::serializer;
///
/// let json_content = r#"
/// {"flags": "LS1oZWxw"}
/// "#;
///
/// let flags = serializer::deserialize_flags(json_content).unwrap();
///
/// let expected_flags = "LS1oZWxw";
/// assert_eq!(expected_flags.as_bytes(), flags.flags());
/// ```
///
pub fn deserialize_flags(subject: &str) -> Result<Flags> {
    let flags: Flags = serde_json::from_str(subject)?;

    Ok(flags)
}
