use serde::Deserialize;
use super::version_data::VersionData;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: String,
    pub content: RequestContent,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum RequestContent {
    Version(VersionData)
}
