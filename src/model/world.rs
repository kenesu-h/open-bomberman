use crate::{
  common::direction::Direction,
  model::{
    bomb::{Flame, FlameImpl, Blast, BlastImpl, Bomb, BombImpl},
    player::{Player},
    stage::{Tile, Stage, StageImpl}
  }
};
use ndarray::{Array, arr2, Ix2};
use std::convert::TryFrom;

/* A struct representing a game world for Bomberman.
 *
 */

pub trait World {
  fn tick(&self) -> Box<dyn World>;

  fn update(&self, tick: i8) -> Box<dyn World>;

  fn move_player(&self, player: &Player, direction: &Direction) -> Box<dyn World>;

  fn tick_bombs(&self) -> Box<dyn World>;

  fn tick_blasts(&self) -> Box<dyn World>;

  fn check_bombs(&self) -> Box<dyn World>;
}

pub struct WorldImpl {
  stage: Box<dyn Stage>,

  players: Vec<Player>,
  bombs: Vec<Box<dyn Bomb>>,
  blasts: Vec<Box<dyn Blast>>
}

impl WorldImpl {
  pub fn new(
    stage: Box<dyn Stage>, players: Vec<Player>,
    bombs: Vec<Box<dyn Bomb>>, blasts: Vec<Box<dyn Blast>>
  ) -> WorldImpl {
    return WorldImpl {
      stage: stage,
      players: players,
      bombs: bombs,
      blasts: blasts
    }
  }

  fn is_wall_or_oob(&self, position: &(i8, i8)) -> bool {
    match self.stage.get_tile(position) {
      Ok(tile) => return tile.is_wall(),
      Err(_) => true
    }
  }

  fn tick_all_blasts(&self) -> Vec<Box<dyn Blast>> {
    let new_blasts: Vec<Box<dyn Blast>> = vec!();
    for blast in self.blasts {
      new_blasts.push(blast.tick(self.flames_hit_wall(blast)));
    }
    return new_blasts;
  }

  fn flames_hit_wall(&self, blast: Box<dyn Blast>) -> Vec<bool> {
    let hit_wall: Vec<bool> = vec!();
    for position in blast.next_positions() {
      hit_wall.push(self.is_wall_or_oob(&position));
    }
    return hit_wall;
  }

  fn clone(&self) -> Box<dyn World> {
    return Box::new(
      WorldImpl {
        stage: self.stage,
        players: self.players,
        bombs: self.bombs,
        blasts: self.blasts
      }
    )
  }
}

impl World for WorldImpl {
  fn tick(&self) -> Box<dyn World> {
    return self
      .tick_bombs()
      .tick_blasts()
      .check_bombs();
  }

  fn update(&self, dt: i8) -> Box<dyn World> {
    let mut new_world: Box<dyn World> = self.clone();
    for i in 0..dt {
      new_world = new_world.tick();
    }
    return new_world;
  }

  fn move_player(&self, player: &Player, direction: &Direction) -> Box<dyn World> {
    return Box::new(
      WorldImpl {
        stage: self.stage,
        players: self.players.into_iter().map(|p| {
          if p == *player {
            return p.set_direction(*direction).set_next_position();
          } else {
            return p;
          }
        }).collect(),
        bombs: self.bombs,
        blasts: self.blasts
      }
    )
  }

  fn tick_bombs(&self) -> Box<dyn World> {
    return Box::new(
      WorldImpl {
        stage: self.stage.copy(),
        players: self.players.clone(),
        bombs: self.bombs.into_iter().map(|b| {
          b.tick()
        }).collect(),
        blasts: self.blasts
      }
    )
  }

  fn tick_blasts(&self) -> Box<dyn World> {
    return Box::new(
      WorldImpl {
        stage: self.stage.copy(),
        players: self.players.clone(),
        bombs: self.bombs,
        blasts: self.tick_all_blasts()
      }
    )
  }

  fn check_bombs(&self) -> Box<dyn World> {
    let mut new_stage: Box<dyn Stage> = self.stage.copy();
    let mut new_bombs: Vec<Box<dyn Bomb>> = vec!();
    let mut new_blasts: Vec<Box<dyn Blast>> = vec!();
    for bomb in self.bombs {
      if bomb.get_lifetime() == &0 {
        let center: (i8, i8) = *bomb.get_position();
        let up_point: (i8, i8) = (center.0, center.1 + 1);
        let down_point: (i8, i8) = (center.0, center.1 - 1);
        let left_point: (i8, i8) = (center.0 - 1, center.1);
        let right_point: (i8, i8) = (center.0 + 1, center.1);

        let up_free: bool = !self.is_wall_or_oob(&up_point);
        let down_free: bool = !self.is_wall_or_oob(&down_point);
        let left_free: bool = !self.is_wall_or_oob(&left_point);
        let right_free: bool = !self.is_wall_or_oob(&right_point);

        if !up_free { new_stage = new_stage.set_tile(&up_point, Tile::Ground) }
        if !down_free { new_stage = new_stage.set_tile(&down_point, Tile::Ground) }
        if !left_free { new_stage = new_stage.set_tile(&left_point, Tile::Ground) }
        if !right_free { new_stage = new_stage.set_tile(&right_point, Tile::Ground) }

        new_blasts.push(
          Box::new(
            BlastImpl::new(
              center,
              *bomb.get_range(),
              up_free,
              down_free,
              left_free,
              right_free
            )
          )
        );
      } else {
        new_bombs.push(bomb);
      }
    }

    return Box::new(
      WorldImpl {
        stage: new_stage,
        players: self.players.clone(),
        bombs: new_bombs,
        blasts: new_blasts
      }
    )
  }
}
