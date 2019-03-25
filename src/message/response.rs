use super::register_data::RegisterData;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: String,
    pub content: ResponseContent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ResponseContent {
    Register(RegisterData),
}
