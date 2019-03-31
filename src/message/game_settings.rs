use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct GameSettings {
    pub id: String,
    pub max_players: u32,
    pub map_width: u32,
    pub map_height: u32,
}
