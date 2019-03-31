use super::generate_map;
use super::message::{
    client_message::ClientMessageContent,
    client_response::ClientResponse,
    game_data::GameData,
    game_settings::GameSettings,
    id::{GameId, PlayerId},
};
use super::settings::Settings;

use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

pub struct Game {
    pub id: GameId,
    pub data: GameData,
}

impl Game {
    pub fn load(id: &GameId, path: &Path) -> Result<Game, io::Error> {
        let mut data_file = PathBuf::from(path);
        data_file.push("data");
        data_file.set_extension("json");
        let file = File::open(data_file)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(Game {
            id: id.clone(),
            data,
        })
    }

    pub fn create(id: &GameId, path: &Path, settings: &GameSettings) -> Result<Game, io::Error> {
        let map = generate_map::generate(&Settings {
            width: settings.map_width,
            height: settings.map_height,
        });
        let data = GameData {
            width: settings.map_width,
            height: settings.map_height,
            tiles: map,
        };
        let game = Game {
            id: id.clone(),
            data,
        };
        game.save(path)?;
        Ok(game)
    }

    pub fn save(&self, path: &Path) -> Result<(), io::Error> {
        fs::create_dir_all(path)?;
        let mut data_file = PathBuf::from(path);
        data_file.push("data");
        data_file.set_extension("json");
        let serialized = serde_json::to_string(&self.data).unwrap();
        fs::write(data_file, serialized)?;
        Ok(())
    }

    pub fn handle_message(
        &mut self,
        _player_id: &PlayerId,
        msg: &ClientMessageContent,
    ) -> ClientResponse {
        match msg {
            ClientMessageContent::GetData => ClientResponse::GameData(self.data.clone()),
        }
    }
}
