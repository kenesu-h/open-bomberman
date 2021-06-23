use ndarray::{Array, Ix2};
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
  Ground, SoftWall, HardWall
}

impl Tile {
  pub fn is_wall(&self) -> bool {
    match self {
      Tile::Ground => return false,
      Tile::SoftWall => return true,
      Tile::HardWall => return true
    }
  }
}

pub fn get_tile(tiles: &Array<Tile, Ix2>, position: &(i8, i8)) -> Tile {
  let usize_position: (usize, usize) =
    (usize::try_from(position.0).unwrap(), usize::try_from(position.1).unwrap());
  return tiles[[usize_position.1, usize_position.0]].clone();
}

/* A struct representing a stage.
 * Stages should only be 15 by 9 at max.
 */
pub trait Stage {
	fn copy(&self) -> Box<dyn Stage>;

  fn get_tiles(&self) -> &Array<Tile, Ix2>;

  fn get_tile(&self, position: &(i8, i8)) -> Result<Tile, &str>;

  fn set_tile(&self, position: &(i8, i8), tile: Tile) -> Box<dyn Stage>;
}

pub struct StageImpl {
  dimensions: (i8, i8),
  tiles: Array<Tile, Ix2>
}

fn get_dimensions(tiles: &Array<Tile, Ix2>) -> (i8, i8) {
  let shape: &[usize] = tiles.shape();
  return (i8::try_from(shape[0]).unwrap(), i8::try_from(shape[1]).unwrap());
}

impl StageImpl {
  fn new(tiles: Array<Tile, Ix2>) -> StageImpl {
    return StageImpl {
      dimensions: get_dimensions(&tiles),
      tiles: tiles
    }
  }

  fn out_of_bounds(&self, position: &(i8, i8)) -> bool {
    return position.0 >= 0 && position.0 <= self.dimensions.0 - 1
      && position.1 >= 0 && position.1 <= self.dimensions.1 - 1;
  }

  fn get_usize_position(&self, position: &(i8, i8)) -> (usize, usize) {
    return (usize::try_from(position.0).unwrap(), usize::try_from(position.1).unwrap());
  }
}

impl Stage for StageImpl {
	fn copy(&self) -> Box<dyn Stage> {
		return Box::new(
			StageImpl {
				dimensions: self.dimensions,
				tiles: self.tiles.clone()
			}
		)
	}

  fn get_tiles(&self) -> &Array<Tile, Ix2> {
    return &self.tiles;
  }

  fn get_tile(&self, position: &(i8, i8)) -> Result<Tile, &str> {
    if !self.out_of_bounds(position) {
      let usize_position: (usize, usize) = self.get_usize_position(position);
      return Ok(self.tiles[[usize_position.1, usize_position.0]]);
    } else {
      return Err("Position is out of bounds.");
    }
  }

  fn set_tile(&self, position: &(i8, i8), tile: Tile) -> Box<dyn Stage> {
    let usize_position: (usize, usize) = self.get_usize_position(position);
    let mut new_tiles: Array<Tile, Ix2> = self.tiles.clone();
    new_tiles[[usize_position.1, usize_position.0]] = tile;

    return Box::new(
      StageImpl {
        dimensions: self.dimensions,
        tiles: new_tiles
      }
    );
  }
}
