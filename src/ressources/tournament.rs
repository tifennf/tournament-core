use std::hash::Hash;

use serde::{Deserialize, Serialize};
use tracing::log::debug;

use super::{
    components::{DiscordName, PlayerList, Pool},
    player::RiotPlayer,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitTournament {
    pool_list: Vec<Pool>,
    player_list: PlayerList,
}

impl InitTournament {
    pub fn new(pool_list: Vec<Pool>, player_list: PlayerList) -> Self {
        InitTournament {
            pool_list,
            player_list,
        }
    }

    pub async fn give_points(&mut self) {
        for pool in self.pool_list.iter() {
            let player = pool.player_list.iter().next().unwrap();

            let api_key = "RGAPI-c1cf5b58-fa9d-44fc-9041-142b7b5aa151";

            let url = format!(
                "https://euw1.api.riotgames.com/tft/summoner/v1/summoners/by-name/{}?api_key={}",
                player.league_name, api_key
            );

            let res = reqwest::get(url).await.unwrap();

            let riot_player = res.json::<RiotPlayer>().await.unwrap();

            let url = format!(
                "https://euw1.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?api_key={}",
                &riot_player.puuid, api_key
            );
            let res = reqwest::get(url).await.unwrap();

            // EUW1_5686910210
        }
    }
}
