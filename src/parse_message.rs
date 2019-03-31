use super::engine::Engine;
use super::message::{
    register_data::RegisterData,
    request::{Request, RequestContent},
    response::{Response, ResponseContent},
};
use std::io::Error;

pub fn parse_message(msg: &str, engine: &mut Engine) -> Result<String, Error> {
    let request = serde_json::from_str::<Request>(msg)?;
    let data = request.content;
    let response = match data {
        RequestContent::Version(data) => {
            println!("Server version {}", &data.version);
            Response {
                id: request.id,
                content: ResponseContent::Register(RegisterData {
                    version: engine.version.clone(),
                    name: engine.identity.name.to_owned(),
                    id: engine.identity.id.to_owned(),
                    admin_token: engine.identity.admin_token.to_owned(),
                    auth_token: engine.identity.auth_token.to_owned(),
                }),
            }
        }
        RequestContent::CreateGame(settings) => {
            println!("Creating a new game");
            let result = engine.game_manager.create_game(&settings);
            let response = if result.is_ok() {
                ResponseContent::Success
            } else {
                let error = result.err().unwrap();
                println!("Cannot create new game: {}", error);
                ResponseContent::Failure(format!("{}", error))
            };
            Response {
                id: request.id,
                content: response,
            }
        }
        RequestContent::ClientMessage(client_msg) => {
            let res = engine.game_manager.handle_message(&client_msg);
            match res {
                Ok(r) => Response {
                    id: request.id,
                    content: ResponseContent::ClientResponse(r),
                },
                Err(err) => {
                    println!("Cannot create new game: {}", err);
                    Response {
                        id: request.id,
                        content: ResponseContent::Failure(format!("{}", err)),
                    }
                }
            }
        }
    };
    let response = serde_json::to_string(&response)?;
    Ok(response)
}
