use anyhow::Result;
use base64::decode;
use std::str;

pub fn base64_decode<T: AsRef<[u8]>>(input: T) -> Result<String> {
    let decoded_body = decode(input).unwrap();
    Ok(str::from_utf8(&decoded_body[..]).unwrap().to_string())
}
