use super::message::game_settings::GameSettings;
use std::io;
use std::path::Path;

pub struct Game {}

impl Game {
    pub fn load(_path: &Path) -> Result<Game, io::Error> {
        Ok(Game {})
    }

    pub fn create(_settings: &GameSettings) -> Game {
        Game {}
    }
}
