use super::game::Game;
use super::message::{
    client_message::ClientMessage, client_response::ClientResponse, game_settings::GameSettings,
    id::GameId,
};
use std::fmt::{self, Display};
use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

pub struct GameManager {
    games: Vec<Game>,
    data_dir: PathBuf,
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
                let id: GameId = dir.file_name().unwrap().to_string_lossy().to_string();
                Game::load(&id, &dir)
                    .map_err(|err| {
                        println!(
                            "Cannot load game {}: {}",
                            dir.file_name().unwrap().to_string_lossy(),
                            err
                        )
                    })
                    .ok()
            })
            .collect();

        println!("Loaded {} games", games.len());

        Ok(GameManager {
            games,
            data_dir: game_data_path,
        })
    }

    pub fn create_game(&mut self, settings: &GameSettings) -> Result<(), io::Error> {
        let mut game_dir = PathBuf::from(&self.data_dir);
        let game_id: GameId = settings.id.to_owned();
        game_dir.push(&game_id);
        let game = Game::create(&game_id, &game_dir, settings)?;
        self.games.push(game);
        Ok(())
    }

    pub fn delete_game(&mut self, game_id: &GameId) -> Result<(), io::Error> {
        let index = self
            .games
            .iter()
            .position(|game| game.id == game_id.to_owned());
        if index.is_none() {
            return Ok(());
        }
        let index = index.unwrap();

        let mut game_dir = PathBuf::from(&self.data_dir);
        game_dir.push(&game_id);
        fs::remove_dir_all(game_dir)?;

        self.games.remove(index);
        Ok(())
    }

    pub fn handle_message(
        &mut self,
        msg: &ClientMessage,
    ) -> Result<Option<ClientResponse>, GameNotFoundError> {
        let player_id = &msg.player_id;
        let game_id = &msg.game_id;
        let game = self
            .games
            .iter_mut()
            .find(|g| g.id == *game_id)
            .ok_or(GameNotFoundError {
                game_id: game_id.clone(),
            })?;
        Ok(Some(game.handle_message(&player_id, &msg.content)))
    }
}

pub struct GameNotFoundError {
    game_id: GameId,
}

impl Display for GameNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {} not found", self.game_id)
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
