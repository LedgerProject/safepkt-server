use anyhow::Result;
use color_eyre::Report;
use hyper::{Body, Request};
use routerify::prelude::*;
use std::convert::Infallible;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    Ok(())
}

pub async fn log_handler(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}
