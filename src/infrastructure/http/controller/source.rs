use crate::infrastructure as infra;
use hyper::{body, Body, Request, Response};
use infra::service::decoder::base64_decode;
use infra::service::file_system::save_content_on_file_system;
use infra::service::serializer;
use std::convert::Infallible;
use std::str;

pub async fn save_source(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (_, request_body) = req.into_parts();
    let body_bytes = &body::to_bytes(request_body).await.unwrap().to_vec()[..];

    let json = str::from_utf8(&body_bytes[..].to_vec())
        .unwrap()
        .to_string();
    let deserialized_json = serializer::deserialize(json.as_str())
        .expect("Can not deserialize request body (expecting valid JSON).");
    let source = deserialized_json.source();

    save_content_on_file_system(source).expect("Can not save content in the file system.");

    Ok(Response::new(Body::from(base64_decode(source).unwrap())))
}
