use hyper::{Body, Request, Response, Method, StatusCode, body};
use base64::decode;
use std::str;
use anyhow::Result;
use crate::infrastructure::service::file_system::save_content_on_file_system;
use crate::infrastructure::service::serializer;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/source") => {
            let (_, request_body) = req.into_parts();
            let body_bytes = &body::to_bytes(request_body).await.unwrap().to_vec()[..];

            let json = str::from_utf8(&body_bytes[..].to_vec()).unwrap().to_string();
            let deserialized_json = serializer::deserialize(json.as_str())?;
            let source = deserialized_json.source();

            save_content_on_file_system(source)?;

            let decoded_body = decode(source).unwrap();
            let source = str::from_utf8(&decoded_body[..]).unwrap().to_string();

            *response.body_mut() = Body::from(source);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}
