use crate::infrastructure as infra;
use crate::infrastructure::http::controller::logs::{
    get_container_logs, get_static_analysis_status,
};
use hyper::{Body, Response, StatusCode};
use infra::http::controller::source::save_source;
use infra::http::controller::static_analysis::start_static_analysis;
use infra::service::logger::logger;
use routerify::{Middleware, RequestInfo, Result, Router, RouterService};
use std::convert::Infallible;
use tracing::error;

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
        .post("/source", save_source)
        .post("/static-analysis/:sourceHash", start_static_analysis)
        .get("/static-analysis/logs/:sourceHash", get_container_logs)
        .get(
            "/static-analysis/status/:sourceHash",
            get_static_analysis_status,
        )
        .err_handler_with_info(error_handler)
        .build()
        .unwrap();

    RouterService::new(router)
}
