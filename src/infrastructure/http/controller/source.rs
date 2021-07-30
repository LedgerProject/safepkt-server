use base64::decode;
use hyper::{Body, Request, Response, body};
use std::str;
use std::convert::Infallible;
use crate::infrastructure::service::file_system::save_content_on_file_system;
use crate::infrastructure::service::serializer;

pub async fn new_source(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (_, request_body) = req.into_parts();
    let body_bytes = &body::to_bytes(request_body).await.unwrap().to_vec()[..];

    let json = str::from_utf8(&body_bytes[..].to_vec()).unwrap().to_string();
    let deserialized_json = serializer::deserialize(json.as_str())
        .expect("Can not deserialize request body (expecting valid JSON).");
    let source = deserialized_json.source();

    save_content_on_file_system(source)
        .expect("Can not save content in the file system.");

    let decoded_body = decode(source).unwrap();
    let source = str::from_utf8(&decoded_body[..]).unwrap().to_string();

    Ok(Response::new(Body::from(source)))
}