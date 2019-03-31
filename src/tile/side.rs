use serde::{Deserialize, Serialize};

#[allow(dead_code, non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone)]
pub enum Side {
    NORTH_WEST,
    NORTH_EAST,
    EAST,
    SOUTH_EAST,
    SOUTH_WEST,
    WEST,
}
