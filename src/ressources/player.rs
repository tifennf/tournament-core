use super::components::DiscordName;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub league_name: String,
    pub discord_username: String,
    pub tag: u16,
    pub discord_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct PlayerVerified {
    pub league_name: String,
    pub discord_name: DiscordName,
    pub discord_id: String,
}

impl Hash for PlayerVerified {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.discord_id.hash(state);
    }
}

impl PartialEq for PlayerVerified {
    fn eq(&self, other: &Self) -> bool {
        self.discord_id == other.discord_id
    }
}

impl TryFrom<Player> for PlayerVerified {
    type Error = Player;

    fn try_from(value: Player) -> Result<Self, Self::Error> {
        let Player {
            discord_username: discord_name,
            tag,
            discord_id,
            league_name,
        } = value.clone();

        let discord_name = DiscordName::new(discord_name, tag).map_err(|_| value)?;

        let player = Self {
            league_name,
            discord_name,
            discord_id,
        };

        Ok(player)
    }
}

#[derive(Debug, Deserialize)]
pub struct RiotPlayer {
    pub puuid: String,
    #[serde(rename = "name")]
    pub league_name: String,
}
