use anyhow::Result;
use color_eyre::Report;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use tracing::{error, info};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use safepkt_server::infrastructure::service::logger;
use safepkt_server::infrastructure::service::router;

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

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(router::handle_request))
    });

    info!("About to listen to address {} and port {}", host_ip_address, port);
    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}