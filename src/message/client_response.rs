use super::game_data::GameData;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ClientResponse {
    GameData(GameData),
}
