use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{PLAYER_AMOUNT, POOL_AMOUNT};

use super::player::PlayerVerified;

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
    pub player_list: HashSet<PlayerVerified>,

    id: usize,
    amount: PoolAmount,
}

impl Pool {
    pub fn new(id: usize, amount: usize) -> Pool {
        let amount = PoolAmount(amount);

        Pool {
            player_list: HashSet::new(),
            id,
            amount,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAmount(pub usize);

impl PlayerAmount {
    pub fn new(amount: usize) -> Result<Self, &'static str> {
        if !PLAYER_AMOUNT.contains(&amount) {
            return Err("Invalid player amount");
        }

        Ok(Self(amount))
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerList {
    pub list: HashSet<PlayerVerified>,
    pub max_amount: PlayerAmount,
    pub current_amount: usize,
}

impl PlayerList {
    pub fn new(max_amount: usize) -> Result<Self, &'static str> {
        let amount = PlayerAmount::new(max_amount)?;
        let list = HashSet::new();
        let current_amount = list.len();

        Ok(Self {
            list,
            max_amount: amount,
            current_amount,
        })
    }

    pub fn insert(&mut self, player: PlayerVerified) -> bool {
        let max_len = self.max_amount.0;
        let list_len = self.list.len();

        let condition = list_len < max_len && self.list.insert(player);
        if condition {
            self.current_amount = self.list.len()
        }

        condition
    }

    pub fn remove(&mut self, player: PlayerVerified) -> bool {
        let condition = self.list.remove(&player);
        if condition {
            self.current_amount = self.list.len()
        }

        condition
    }
}
