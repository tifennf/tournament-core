use std::{collections::HashSet, slice::Chunks};

use serde::{Deserialize, Serialize};

use crate::{PLAYER_AMOUNT, POOL_AMOUNT, POOL_SIZE};

use super::player::{MatchPlayer, Player};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DiscordName {
    pub name: String,
    pub tag: String,
}

impl DiscordName {
    pub fn new(name: String, tag: u16) -> Result<Self, &'static str> {
        if tag > 9999 {
            return Err("Invalid discord tag");
        }

        let tag = if tag < 1000 {
            let mut tag = tag.to_string();

            let mut i = 0;

            while tag.len() < 4 {
                tag.insert(i, '0');
                i += 1;
            }

            tag
        } else {
            tag.to_string()
        };

        Ok(Self { name, tag })
    }

    pub fn get_full_name(&self) -> String {
        format!("{}#{}", self.name, self.tag)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub list: HashSet<Player>,
    pub id: usize,
    pub amount: PoolAmount,
    pub round: u8,
}

impl Pool {
    pub fn new(id: usize, amount: usize, round: u8) -> Pool {
        let amount = PoolAmount(amount);

        Pool {
            list: HashSet::new(),
            id,
            amount,
            round,
        }
    }

    pub fn from_slice(player_slice: &[Player], id: usize, round: u8) -> Pool {
        let amount = PoolAmount(POOL_SIZE);

        let list = player_slice.iter().map(|p| p.clone()).collect();

        Pool {
            list,
            amount,
            id,
            round,
        }
    }

    pub fn contains_puuid(&self, puuid: &str) -> bool {
        self.list.iter().any(|player| player.puuid == puuid)
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PlayerAmount(pub usize);

// impl PlayerAmount {
//     pub fn new(amount: usize) -> Result<Self, &'static str> {
//         if !PLAYER_AMOUNT.contains(&amount) {
//             return Err("Invalid player amount");
//         }

//         Ok(Self(amount))
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAmount(usize);

impl PoolAmount {
    pub fn new(amount: usize) -> Result<Self, &'static str> {
        if !POOL_AMOUNT.contains(&amount) {
            return Err("Invalid pool amount");
        }

        Ok(Self(amount))
    }
}

#[derive(Debug, Deserialize)]
pub struct MatchMetadata {
    data_version: String,
    match_id: String,
    pub participants: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatchInfo {
    pub participants: Vec<MatchPlayer>,
}

#[derive(Debug, Deserialize)]
pub struct MatchData {
    pub metadata: MatchMetadata,
    pub info: MatchInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Placement {
    pub place: u8,
    pub round: u8,
}

impl Placement {
    pub fn new(place: u8, round: u8) -> Self {
        Placement { place, round }
    }
}
