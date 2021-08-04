use anyhow::Result;
use color_eyre::Report;
use hyper::Server;
use safepkt_server::app::middleware;
use safepkt_server::app::router;
use safepkt_server::infra::signal_handling;
use std::env;
use std::net::SocketAddr;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Report> {
    middleware::logger::setup()?;

    dotenv::dotenv().ok();

    let host = env::var("HOST")?;
    let port = env::var("PORT")?;

    let host_ip_address = host.as_str();
    let port = port.as_str();

    let hostname_port = format!("{}:{}", host_ip_address, port);
    let addr: SocketAddr = hostname_port.as_str().parse()?;

    let router = router::new_router().unwrap();

    info!(
        "About to listen to address {} and port {}",
        host_ip_address, port
    );
    let server = Server::bind(&addr).serve(router);

    let graceful = server.with_graceful_shutdown(signal_handling::handle_shutdown_signal());

    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }

    Ok(())
}
