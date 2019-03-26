use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ClientMessage {
    GetData,
}
