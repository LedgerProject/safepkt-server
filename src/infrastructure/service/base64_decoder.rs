use anyhow::Result;
use base64::decode as base64_decode;
use std::str;

/// Decode base64 encoded content
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::base64_decoder;
///
/// let decoded_contents = base64_decoder::decode("dGVzdA==".as_bytes()).unwrap();
/// assert_eq!("test", decoded_contents);
/// ```
///
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<String> {
    let decoded_body = base64_decode(input).unwrap();
    Ok(str::from_utf8(&decoded_body[..]).unwrap().to_string())
}

#[test]
fn it_decodes_base64_encoded_content() {
    use crate::infra::base64_decoder;

    let decoded_contents = base64_decoder::decode(b"dGVzdA==").unwrap();
    assert_eq!("test", decoded_contents);
}
