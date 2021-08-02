use crate::infrastructure as infra;
use hyper::{Body, Response, StatusCode};
use infra::http::controller::source::save_source;
use infra::http::controller::{llvm_bitcode_generation, symbolic_execution};
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
        .post(
            "/llvm-bitcode-generation/:targetHash",
            llvm_bitcode_generation::start_running_step,
        )
        .get(
            "/llvm-bitcode-generation/:targetHash/logs",
            llvm_bitcode_generation::get_logs,
        )
        .get(
            "/llvm-bitcode-generation/:targetHash/status",
            llvm_bitcode_generation::get_status,
        )
        .post(
            "/symbolic-execution/:targetHash",
            symbolic_execution::start_running_step,
        )
        .get(
            "/symbolic-execution/:targetHash/logs",
            symbolic_execution::get_logs,
        )
        .get(
            "/symbolic-execution/:targetHash/status",
            symbolic_execution::get_status,
        )
        .err_handler_with_info(error_handler)
        .build()
        .unwrap();

    RouterService::new(router)
}
