use crate::infrastructure as infra;
use hyper::{Body, Response, StatusCode};
use infra::http::controller::source::save_source;
use infra::http::controller::static_analysis;
use infra::service::logger::logger;
use routerify::{Middleware, RequestInfo, Result, Router, RouterService};
use std::convert::Infallible;
use tracing::error;

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("Routing error: {}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Sorry, something went wrong.")))
        .unwrap()
}

pub fn new_router() -> Result<RouterService<Body, Infallible>> {
    let router = Router::builder()
        .middleware(Middleware::pre(logger))
        .post("/source", save_source)
        .post("/static-analysis/:sourceHash", static_analysis::start)
        .get(
            "/static-analysis/logs/:sourceHash",
            static_analysis::get_logs,
        )
        .get(
            "/static-analysis/status/:sourceHash",
            static_analysis::get_status,
        )
        .err_handler_with_info(error_handler)
        .build()
        .unwrap();

    RouterService::new(router)
}
