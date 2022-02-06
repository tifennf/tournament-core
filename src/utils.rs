use std::{
    borrow::BorrowMut,
    cmp::Ordering,
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

fn parse_points(placement: Placement) -> u16 {
    let place = placement.place as usize;

    PLACEMENT_POINTS[place - 1]
}

async fn puuid_url(puuid: &str, count: u8) -> String {
    format!(
        "https://europe.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?count={}&api_key={}",
        puuid, count, API_KEY
    )
}

async fn match_id_url(match_id: String) -> String {
    format!(
        "https://europe.api.riotgames.com/tft/match/v1/matches/{}?api_key={}",
        match_id, API_KEY
    )
}

//give points to each player
async fn give_points(pool: &Pool, match_data: MatchData) -> Result<HashSet<Player>, StatusCode> {
    let match_players = match_data.info.participants;

    let match_players: Vec<MatchPlayer> = match_players
        .into_iter()
        .filter(|p| pool.contains_puuid(&p.puuid))
        .collect();

    if match_players.len() == 0 {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
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

    Ok(player_list)
}

pub async fn parse_pool_points(pool_list: &Vec<Pool>) -> Result<HashSet<Player>, StatusCode> {
    let mut result = Err(StatusCode::INTERNAL_SERVER_ERROR);

    for pool in pool_list {
        let player = pool.list.iter().next();

        match player {
            Some(player) => {
                let url = puuid_url(&player.puuid, 1).await;

                let res = reqwest::get(url).await.unwrap();
                let match_id = res.json::<Vec<String>>().await.unwrap();

                for id in match_id {
                    let url = match_id_url(id).await;

                    let res = reqwest::get(url).await.unwrap();
                    let match_data = res
                        .json::<MatchData>()
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                    result = Ok(give_points(&pool, match_data).await?);
                }
            }
            None => result = Err(StatusCode::NOT_FOUND),
        }
    }

    result
}

pub fn make_pools(player_list: &HashSet<Player>, round: u8) -> Vec<Pool> {
    let player_list: Vec<Player> = player_list.clone().into_iter().collect();

    player_list
        .chunks(POOL_SIZE)
        .enumerate()
        .map(|(id, slice)| Pool::from_slice(slice, id, round))
        .collect()
}

pub fn sort_players(player_list: HashSet<Player>) -> HashSet<Player> {
    let mut player_list: Vec<Player> = player_list.into_iter().collect();

    player_list.sort_by(|p1, p2| p1.points.cmp(&p2.points));

    let player_list: HashSet<Player> = player_list.into_iter().collect();

    player_list
}

fn compare_player_top(p1: Player, p2: Player) -> Ordering {
    todo!()
}
