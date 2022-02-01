use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tracing::log::debug;

use crate::utils::get_match_data;

use super::{
    components::{DiscordName, Pool},
    player::Player,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitTournament {
    pub pool_list: Vec<Pool>,
    pub player_list: HashSet<Player>,
    pub history: Vec<Vec<Pool>>,
}

impl InitTournament {
    pub fn new(pool_list: Vec<Pool>, player_list: HashSet<Player>) -> Self {
        let history = Vec::new();

        InitTournament {
            pool_list,
            player_list,
            history,
        }
    }
}
