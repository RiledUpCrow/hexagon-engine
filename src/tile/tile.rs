use super::ground_feature::GroundFeature;
use super::ground_type::GroundType;
use super::side::Side;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub ground_type: GroundType,
    pub ground_feature: Option<GroundFeature>,
    pub hill: bool,
    pub discovered: bool,
    pub visible: bool,
    pub rivers: Vec<Side>,
}
