use super::super::tile::tile::Tile;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Tile>>,
}
