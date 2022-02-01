use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    fs,
    io::{BufReader, BufWriter, Write},
    sync::{LockResult, MutexGuard},
};

use axum::{http::StatusCode, routing::MethodRouter, Router};
use rand::{prelude::SliceRandom, thread_rng};

use tracing::log::debug;

use crate::{
    ressources::{
        components::{MatchData, Placement, Pool},
        player::{MatchPlayer, Player},
    },
    API_KEY, PLACEMENT_POINTS, POOL_SIZE,
};

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

pub fn parse_points(placement: Placement) -> u16 {
    let place = placement.place as usize;

    PLACEMENT_POINTS[place - 1]
}

pub async fn get_match_id(puuid: &str, count: u8) -> Vec<String> {
    let url = format!(
        "https://europe.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?count={}&api_key={}",
        puuid, count, API_KEY
    );
    let res = reqwest::get(url).await.unwrap();

    let match_id = res.json::<Vec<String>>().await.unwrap();

    match_id
}

pub async fn get_match_data(match_id: String) -> MatchData {
    let url = format!(
        "https://europe.api.riotgames.com/tft/match/v1/matches/{}?api_key={}",
        match_id, API_KEY
    );

    let res = reqwest::get(url).await.unwrap();

    let match_data = res.json::<MatchData>().await.unwrap();

    match_data
}

pub async fn give_points(pool: &Pool, match_data: MatchData) -> Result<Pool, String> {
    let match_players = match_data.info.participants;

    let match_players: Vec<MatchPlayer> = match_players
        .into_iter()
        .filter(|p| pool.contains_puuid(&p.puuid))
        .collect();

    if match_players.len() == 0 {
        return Err("wrong match".to_string());
    }

    let player_list: HashSet<Player> = match_players
        .iter()
        .map(|p| {
            let mut player = pool
                .list
                .clone()
                .into_iter()
                .find(|player| player.puuid == p.puuid)
                .unwrap();

            let placement = Placement::new(p.placement, pool.round.clone());
            let points = parse_points(placement);

            player.points += points;

            player
        })
        .collect();

    let mut pool = pool.clone();

    pool.list = player_list;

    Ok(pool)
}
