use crate::model::stage::Tile;
use ndarray::{Array, Ix2};
use std::convert::TryFrom;

pub fn get_tile(tiles: &Array<Tile, Ix2>, position: &(i8, i8)) -> Tile {
  let usize_position: (usize, usize) =
    (usize::try_from(position.0).unwrap(), usize::try_from(position.1).unwrap());
  return tiles[[usize_position.1, usize_position.0]].clone();
}
