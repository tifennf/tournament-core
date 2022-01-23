use std::{
    fs,
    io::{BufReader, BufWriter, Write},
    sync::{LockResult, MutexGuard},
};

use axum::{http::StatusCode, routing::MethodRouter, Router};
use rand::{prelude::SliceRandom, thread_rng};
use serde_json::Value;

use crate::{ressources::components::PlayerList, POOL_SIZE};

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

pub fn save_plist(list: &PlayerList) -> Result<(), String> {
    let file = fs::File::create("./players.json").map_err(|err| err.to_string())?;

    let mut writer = BufWriter::new(&file);
    serde_json::to_writer_pretty(&mut writer, list).map_err(|err| err.to_string())?;

    writer.flush().map_err(|err| err.to_string())?;

    Ok(())
}
pub fn get_plist() -> Result<PlayerList, String> {
    let file = fs::File::open("./players.json").map_err(|err| err.to_string())?;

    let reader = BufReader::new(&file);

    serde_json::from_reader(reader).map_err(|err| err.to_string())
}
