use std::net::SocketAddr;

use tournament_core::{config::get_config, run};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tournament-core=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let config = get_config();
    let addr = SocketAddr::from((config.server.ip, config.server.port));
    tracing::debug!("Listening on address: {}", addr);

    run(&addr, config).await;
}
