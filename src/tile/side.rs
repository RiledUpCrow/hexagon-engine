use serde::Serialize;

#[allow(dead_code, non_camel_case_types)]
#[derive(Serialize, Clone)]
pub enum Side {
  NORTH_WEST,
  NORTH_EAST,
  EAST,
  SOUTH_EAST,
  SOUTH_WEST,
  WEST,
}
