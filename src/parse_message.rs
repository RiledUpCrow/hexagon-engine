use serde::{Deserialize, Serialize};
use serde_json::{value::Value, Map};
use std::io::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IncomingMessage {
    r#type: String,
    request_id: String,
    data: Map<String, Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OutcomingMessage {
    response_id: String,
    data: Map<String, Value>,
}

pub fn parse_message(msg: &str) -> Result<String, Error> {
    let mut incoming: IncomingMessage = serde_json::from_str(msg)?;
    let mut val = Value::String(String::from("Unknown"));
    let data = incoming.data.get_mut("data").unwrap_or(&mut val);
    let mut map = Map::new();
    map.insert(String::from("response"), data.take());
    let outcoming = OutcomingMessage {
        response_id: incoming.request_id,
        data: map,
    };
    let response = serde_json::to_string(&outcoming)?;
    Ok(response)
}
