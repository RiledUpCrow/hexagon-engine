use serde::Serialize;

#[derive(Serialize, Clone, PartialEq)]
pub enum GroundType {
    GRASSLAND,
    PLAINS,
    TUNDRA,
    DESERT,
    SNOW,
    OCEAN,
    MOUNTAIN,
}
