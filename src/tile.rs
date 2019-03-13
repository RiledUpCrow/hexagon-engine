use super::ground_type::GroundType;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
  pub ground_type: GroundType,
}
