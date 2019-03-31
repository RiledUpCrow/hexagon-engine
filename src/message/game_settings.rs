use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct GameSettings {
    id: String,
    max_players: u32,
    map_width: u32,
    map_height: u32,
}
