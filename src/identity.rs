use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub name: String,
    pub id: String,
    pub admin_token: String,
    pub auth_token: String,
}

impl Identity {
    pub fn generate() -> Identity {
        Identity {
            name: String::from("Beeeton"),
            id: String::from("hehe"),
            admin_token: String::from("hoho"),
            auth_token: String::from("hyhy"),
        }
    }
}
