use super::engine::Engine;
use super::generate_map;
use super::message::{
    client_message::ClientMessage,
    client_response::ClientResponse,
    game_data::GameData,
    register_data::RegisterData,
    request::{Request, RequestContent},
    response::{Response, ResponseContent},
};
use super::settings::Settings;
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
            engine.game_manager.create_game(&settings);
            Response {
                id: request.id,
                content: ResponseContent::Success,
            }
        }
        RequestContent::ClientMessage(client_msg) => match client_msg {
            ClientMessage::GetData => Response {
                id: request.id,
                content: ResponseContent::ClientResponse(ClientResponse::GameData(GameData {
                    width: 128,
                    height: 80,
                    tiles: generate_map::generate(&Settings {
                        width: 128,
                        height: 80,
                    }),
                })),
            },
        },
    };
    let response = serde_json::to_string(&response)?;
    Ok(response)
}
