use super::game::Game;
use super::message::game_settings::GameSettings;
use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

pub struct GameManager {
    games: Vec<Game>,
}

impl GameManager {
    pub fn new(data_path: &Path) -> Result<GameManager, io::Error> {
        let mut game_data_path = PathBuf::from(data_path);
        game_data_path.push("games");

        if !game_data_path.exists() {
            fs::create_dir_all(&game_data_path)?;
        }

        let games: Vec<Game> = read_game_dirs(&game_data_path)?
            .iter()
            .filter_map(|dir| {
                Game::load(&dir)
                    .map_err(|err| {
                        println!(
                            "Cannot load game {}: {}",
                            data_path.file_name().unwrap().to_string_lossy(),
                            err
                        )
                    })
                    .ok()
            })
            .collect();

        println!("Loaded {} games", games.len());

        Ok(GameManager { games })
    }

    pub fn create_game(&mut self, settings: &GameSettings) {
        let game = Game::create(settings);
        self.games.push(game);
    }
}

fn read_game_dirs(game_data: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let result = fs::read_dir(game_data)?
        .filter_map(|res| {
            res.ok().filter(|dir| {
                dir.file_type()
                    .map(|file_type| file_type.is_dir())
                    .unwrap_or(false)
            })
        })
        .map(|dir| dir.path())
        .collect();

    Ok(result)
}
