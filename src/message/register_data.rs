use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterData {
    pub version: String,
    pub id: String,
    pub admin_token: String,
}