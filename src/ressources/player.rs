use super::components::{DiscordName, Placement};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Player {
    pub league_name: String,
    pub discord_name: DiscordName,
    pub discord_id: String,
    pub riot_account_id: String,
    pub puuid: String,
    #[serde(default)]
    pub points: u16,
    pub placement: Placement,
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.discord_id.hash(state);
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.discord_id == other.discord_id
    }
}

#[derive(Debug, Deserialize)]
pub struct RiotPlayer {
    pub puuid: String,
    #[serde(rename = "name")]
    pub league_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MatchPlayer {
    pub placement: u8,
    pub puuid: String,
}
