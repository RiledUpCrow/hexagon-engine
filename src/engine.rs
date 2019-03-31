use super::game_manager::GameManager;
use super::identity::Identity;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Engine {
    pub game_manager: GameManager,
    pub identity: Identity,
    pub version: String,
}

impl Engine {
    pub fn new(version: &str, data_url: &Path) -> Result<Engine, EngineError> {
        let mut path = PathBuf::from(data_url);
        path.push("identity");
        path.set_extension("json");
        let path = path;

        let identity = fs::read_to_string(&path)
            .map(|string| serde_json::from_str::<Identity>(&string))
            .or_else(|err| {
                if let io::ErrorKind::NotFound = err.kind() {
                    println!("Identity not found, generating a new one...");
                    let id = Identity::generate();
                    let st = serde_json::to_string_pretty(&id)?;
                    path.parent().map(|dir| fs::create_dir_all(&dir));
                    fs::write(&path, st)?;
                    Ok(Ok(id))
                } else {
                    Err(err)
                }
            })??;

        println!("Engine name: {}", &identity.name);
        println!("Engine admin token: {}", &identity.admin_token);

        Ok(Engine {
            game_manager: GameManager::new(data_url)?,
            identity,
            version: String::from(version),
        })
    }
}

#[derive(Debug)]
pub enum EngineError {
    FileProblem(io::Error),
    IncorrectPath,
    InvalidJson,
}

impl From<io::Error> for EngineError {
    fn from(err: io::Error) -> EngineError {
        EngineError::FileProblem(err)
    }
}

impl From<std::string::ParseError> for EngineError {
    fn from(_: std::string::ParseError) -> EngineError {
        EngineError::IncorrectPath
    }
}

impl From<serde_json::error::Error> for EngineError {
    fn from(_: serde_json::error::Error) -> EngineError {
        EngineError::InvalidJson
    }
}
