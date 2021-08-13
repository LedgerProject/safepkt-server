use crate::app;
use crate::infra;
use app::controller;
use hyper::{body, Body, Request, Response, StatusCode};
use infra::file_system::save_content_in_file_system;
use infra::serializer;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str;

pub async fn save_source(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (_, request_body) = req.into_parts();
    let body_bytes = &body::to_bytes(request_body).await.unwrap().to_vec()[..];

    let json = str::from_utf8(&body_bytes[..].to_vec())
        .unwrap()
        .to_string();
    let deserialized_json = serializer::deserialize_source(json.as_str())
        .expect("Can not deserialize request body (expecting valid JSON).");
    let source = deserialized_json.source();

    let (_, project_id) =
        save_content_in_file_system(source).expect("Can not save content in the file system.");

    let mut response = HashMap::<String, String>::new();
    response.insert("project_id".to_string(), project_id);

    controller::build_response(serde_json::to_vec(&response).unwrap(), StatusCode::OK)
}
