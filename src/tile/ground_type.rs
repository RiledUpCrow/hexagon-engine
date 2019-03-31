use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum GroundType {
    GRASSLAND,
    PLAINS,
    TUNDRA,
    DESERT,
    SNOW,
    OCEAN,
    MOUNTAIN,
}
