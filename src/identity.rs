use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

fn rand_str() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(32).collect()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub id: String,
    pub admin_token: String,
    pub auth_token: String,
}

impl Identity {
    pub fn generate() -> Identity {
        Identity {
            id: rand_str(),
            admin_token: rand_str(),
            auth_token: rand_str(),
        }
    }
}
