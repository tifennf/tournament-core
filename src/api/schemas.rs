use std::collections::HashSet;

use serde::Deserialize;

use crate::ressources::{components::Pool, player::Player};

#[derive(Debug, Deserialize)]
pub struct TournamentInfo {
    pub pool_list: Vec<Pool>,
    pub player_list: HashSet<Player>,
}
