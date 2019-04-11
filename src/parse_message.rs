use super::engine::Engine;
use super::message::{
    register_data::RegisterData,
    request::{Request, RequestContent},
    response::{Response, ResponseContent},
};
use std::io::Error;

pub fn parse_message(msg: &str, engine: &mut Engine) -> Result<Option<String>, Error> {
    let request = serde_json::from_str::<Request>(msg)?;
    let data = &request.content;
    let response: Option<Response> = match data {
        RequestContent::Version(data) => {
            println!("Server version {}", &data.version);
            Some(Response {
                id: request.id,
                content: ResponseContent::Register(RegisterData {
                    version: engine.version.clone(),
                    id: engine.identity.id.to_owned(),
                    admin_token: engine.identity.admin_token.to_owned(),
                    auth_token: engine.identity.auth_token.to_owned(),
                }),
            })
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
            Some(Response {
                id: request.id,
                content: response,
            })
        }
        RequestContent::DeleteGame(game_id) => {
            println!("Deleting game {}", game_id);
            let result = engine.game_manager.delete_game(game_id);
            let response = if result.is_ok() {
                ResponseContent::Success
            } else {
                let error = result.err().unwrap();
                println!("Cannot delete game {}: {}", game_id, error);
                ResponseContent::Failure(format!("{}", error))
            };
            Some(Response {
                id: request.id,
                content: response,
            })
        }
        RequestContent::ClientMessage(client_msg) => {
            let res = engine.game_manager.handle_message(&client_msg);
            match res {
                Ok(Some(m)) => Some(Response {
                    id: request.id,
                    content: ResponseContent::ClientResponse(m),
                }),
                Ok(None) => None,
                Err(err) => {
                    println!("Cannot create new game: {}", err);
                    Some(Response {
                        id: request.id,
                        content: ResponseContent::Failure(format!("{}", err)),
                    })
                }
            }
        }
    };
    let response = response.map(|res| serde_json::to_string(&res).unwrap());
    Ok(response)
}
