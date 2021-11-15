use crate::app;
use app::controller;
use app::middleware;
use hyper::{Body, Response, StatusCode};
use routerify::{Middleware, RequestInfo, Result, Router, RouterService};
use routerify_cors::enable_cors_all;
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
        .middleware(Middleware::pre(middleware::logger::log_handler))
        .middleware(enable_cors_all())
        .post("/source", controller::source::save_source)
        .get("/steps", controller::verification_step::get_steps)
        .post(
            "/:stepName/:projectId",
            controller::verification_step::start_running_step,
        )
        .get(
            "/:stepName/:projectId/report",
            controller::verification_step::get_step_report,
        )
        .get(
            "/:stepName/:projectId/progress",
            controller::verification_step::get_step_progress,
        )
        .delete(
            "/:stepName/:projectId",
            controller::verification_step::stop_running_step,
        )
        .err_handler_with_info(error_handler)
        .build()
        .unwrap();

    RouterService::new(router)
}
