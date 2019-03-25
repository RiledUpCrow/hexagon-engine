use super::message::{
    register_data::RegisterData,
    request::{Request, RequestContent},
    response::{Response, ResponseContent},
};
use std::io::Error;

pub fn parse_message(msg: &str) -> Result<String, Error> {
    let request = serde_json::from_str::<Request>(msg)?;
    let data = request.content;
    let response = match data {
        RequestContent::Version(_) => Response {
            id: request.id,
            content: ResponseContent::Register(RegisterData {
                version: String::from("0.1.0"),
                id: String::from("hehe"),
                admin_token: String::from("hoho"),
            }),
        },
    };
    let response = serde_json::to_string(&response)?;
    Ok(response)
}
