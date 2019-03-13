extern crate rand;

use super::settings::Settings;
use super::tile::{ground_feature::GroundFeature, ground_type::GroundType, side::Side, tile::Tile};
use rand::{seq::SliceRandom, thread_rng};

const TYPES: [GroundType; 7] = [
  GroundType::GRASSLAND,
  GroundType::PLAINS,
  GroundType::TUNDRA,
  GroundType::DESERT,
  GroundType::SNOW,
  GroundType::OCEAN,
  GroundType::MOUNTAIN,
];

const FEATURES: [GroundFeature; 1] = [GroundFeature::FOREST];

const WITH_TREES: [GroundType; 2] = [GroundType::GRASSLAND, GroundType::PLAINS];

const WITH_HILLS: [GroundType; 5] = [
  GroundType::GRASSLAND,
  GroundType::PLAINS,
  GroundType::TUNDRA,
  GroundType::DESERT,
  GroundType::SNOW,
];

const SIDES: [Side; 3] = [Side::SOUTH_EAST, Side::SOUTH_WEST, Side::WEST];

pub fn generate(settings: &Settings) -> Vec<Vec<Tile>> {
  let mut rng = thread_rng();
  let mut columns = Vec::new();
  for _x in 0..settings.width {
    let mut row = Vec::new();
    for _y in 0..settings.height {
      let t = TYPES.choose(&mut rng).unwrap();
      let tile = Tile {
        ground_type: t.clone(),
        ground_feature: if WITH_TREES.iter().any(|val| val == t) && rand::random() {
          Some(FEATURES.choose(&mut rng).unwrap().clone())
        } else {
          None
        },
        hill: WITH_HILLS.iter().any(|val| val == t) && rand::random::<f64>() >= 0.75,
        discovered: true,
        visible: true,
        rivers: SIDES
          .iter()
          .filter(|_| (rand::random::<f64>() >= 0.9))
          .map(|r| r.clone())
          .collect(),
      };
      row.push(tile);
    }
    columns.push(row);
  }
  columns
}
