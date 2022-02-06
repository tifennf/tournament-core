use std::net::SocketAddr;

use tournament_core::run;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tournament_service=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3024));
    tracing::debug!("Listening on address: {}", addr);

    run(&addr).await;
}
