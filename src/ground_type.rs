use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum GroundType {
  GRASSLAND,
  PLAINS,
  TUNDRA,
  DESERT,
  SNOW,
  OCEAN,
  MOUNTAIN,
}
