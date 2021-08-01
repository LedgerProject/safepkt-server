use anyhow::Result;
use color_eyre::Report;
use hyper::Server;
use infra::http::router::new_router;
use infra::service::logger;
use infra::signal::shutdown;
use safepkt_server::infrastructure as infra;
use std::env;
use std::net::SocketAddr;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Report> {
    logger::setup()?;

    dotenv::dotenv().ok();

    let host = env::var("HOST")?;
    let port = env::var("PORT")?;

    let host_ip_address = host.as_str();
    let port = port.as_str();

    let hostname_port = format!("{}:{}", host_ip_address, port);
    let addr: SocketAddr = hostname_port.as_str().parse()?;

    let router = new_router().unwrap();

    info!(
        "About to listen to address {} and port {}",
        host_ip_address, port
    );
    let server = Server::bind(&addr).serve(router);

    let graceful = server.with_graceful_shutdown(shutdown::shutdown_signal());

    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }

    Ok(())
}
