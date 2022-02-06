use std::{net::SocketAddr, sync::Arc};

use api::routes::{info, init, next_round};
use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use ressources::tournament::Tournament;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

pub mod api;
pub mod ressources;
pub mod utils;

pub const POOL_AMOUNT: [usize; 4] = [1, 2, 4, 8];
pub const PLAYER_AMOUNT: [usize; 4] = [8, 16, 32, 64];
pub const PLACEMENT_POINTS: [u16; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
pub const POOL_SIZE: usize = 8;
pub const API_KEY: &str = "RGAPI-c1cf5b58-fa9d-44fc-9041-142b7b5aa151";

pub async fn run(addr: &SocketAddr) {
    let state = Arc::new(Mutex::new(None::<Tournament>));

    let app = Router::new()
        .route("/info", get(info))
        .route("/init", post(init))
        .route("/next", get(next_round))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
