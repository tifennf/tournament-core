use std::sync::Arc;

use axum::{extract::Extension, Json};
use reqwest::StatusCode;
use tokio::sync::Mutex;

use crate::{
    ressources::tournament::{Tournament, TournamentMetadata},
    utils::{make_pools, parse_pool_points, sort_players},
};

use super::schemas::TournamentInfo;

type State = Arc<Mutex<Option<Tournament>>>;

pub async fn init(
    Extension(state): Extension<State>,
    Json(data): Json<TournamentInfo>,
) -> Json<Option<Tournament>> {
    let metadata = TournamentMetadata {
        name: "test".to_string(),
        player_amount: data.player_list.len(),
        pool_amount: data.pool_list.len(),
    };

    let tournament = Tournament::init(metadata, data.pool_list, data.player_list);

    let mut state = state.lock().await;

    *state = Some(tournament);

    Json(state.clone())
}
pub async fn next_round(
    Extension(state): Extension<State>,
) -> Result<Json<Tournament>, StatusCode> {
    let mut state = state.lock().await;

    if let Some(ref mut tournament) = *state {
        tournament.round += 1;

        let player_list = parse_pool_points(&tournament.pool_list)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let player_list = sort_players(player_list);

        let pool_list = make_pools(&player_list, tournament.round);

        tournament.update(player_list);

        tournament.pool_list = pool_list;

        Ok(Json(tournament.clone()))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
pub async fn info(Extension(state): Extension<State>) -> Json<Option<Tournament>> {
    let state = state.lock().await;

    Json(state.clone())
}
