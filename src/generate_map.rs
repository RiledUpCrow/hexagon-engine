use super::settings::Settings;
use super::tile::Tile;

pub fn log(settings: &Settings) -> Vec<Vec<Tile>> {
  let mut columns = Vec::new();
  for _x in 0..settings.width {
    let mut row = Vec::new();
    for _y in 0..settings.height {
      let tile = Tile {
        gt: "grass".to_owned(),
      };
      row.push(tile);
    }
    columns.push(row);
  }
  columns
}
