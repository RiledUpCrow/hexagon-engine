use super::id::{GameId, PlayerId};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMessage {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub content: ClientMessageContent,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ClientMessageContent {
    GetData,
}
