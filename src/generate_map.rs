extern crate rand;

use super::ground_type::GroundType;
use super::settings::Settings;
use super::tile::Tile;
use rand::{seq::SliceRandom, thread_rng};

pub fn log(settings: &Settings) -> Vec<Vec<Tile>> {
  let mut rng = thread_rng();
  let mut columns = Vec::new();
  for _x in 0..settings.width {
    let mut row = Vec::new();
    for _y in 0..settings.height {
      let types = [
        GroundType::GRASSLAND,
        GroundType::PLAINS,
        GroundType::TUNDRA,
        GroundType::DESERT,
        GroundType::SNOW,
        GroundType::OCEAN,
        GroundType::MOUNTAIN,
      ];
      let tile = Tile {
        ground_type: types.choose(&mut rng).unwrap().clone(),
      };
      row.push(tile);
    }
    columns.push(row);
  }
  columns
}
