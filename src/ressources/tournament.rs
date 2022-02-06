use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use tracing::log::debug;

use super::{
    components::{DiscordName, MatchData, Pool},
    player::Player,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentMetadata {
    pub name: String,
    pub player_amount: usize,
    pub pool_amount: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub round: u8,
    pub pool_list: Vec<Pool>,
    pub player_list: HashSet<Player>,
    pub history: Vec<Vec<Pool>>,
    pub metadata: TournamentMetadata,
}

impl Tournament {
    pub fn init(
        metadata: TournamentMetadata,
        pool_list: Vec<Pool>,
        player_list: HashSet<Player>,
    ) -> Self {
        let history = Vec::new();

        Tournament {
            round: 1,
            pool_list,
            player_list,
            history,
            metadata,
        }
    }

    pub fn add_points(&mut self, player_list: HashSet<Player>) {
        self.history.push(self.pool_list.clone());
        self.player_list = player_list;
    }

    // pub fn next_round(&mut self, pool)
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum TournamentState {
//     Init,
//     Running(u8),
//     Done,
// }
