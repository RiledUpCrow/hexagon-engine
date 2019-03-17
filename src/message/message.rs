use super::register_data::RegisterData;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum Message {
    Register(RegisterData)
}