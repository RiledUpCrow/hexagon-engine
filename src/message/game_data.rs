use super::super::tile::tile::Tile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Tile>>,
}
