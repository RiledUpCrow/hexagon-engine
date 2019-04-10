use super::client_message::ClientMessage;
use super::game_settings::GameSettings;
use super::id::GameId;
use super::version_data::VersionData;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: String,
    pub content: RequestContent,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum RequestContent {
    Version(VersionData),
    CreateGame(GameSettings),
    DeleteGame(GameId),
    ClientMessage(ClientMessage),
}
