use hyper::{Body, Response, StatusCode};
use std::convert::Infallible;
use tracing::error;
use routerify::{Middleware, Router, RequestInfo, Result, RouterService};
use crate::infrastructure as infra;
use infra::http::controller::source::new_source;
use infra::http::controller::static_analysis::new_static_analysis;
use infra::service::logger::logger;

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("router error: {}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

pub fn new_router() -> Result<RouterService<Body, Infallible>> {
    let router = Router::builder()
        .middleware(Middleware::pre(logger))
        .post("/source", new_source)
        .post("/static-analysis/:sourceHash", new_static_analysis)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap();

    RouterService::new(router)
}