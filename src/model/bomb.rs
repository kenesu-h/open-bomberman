use crate::{
  common::direction::Direction,
  model::{
    stage::Tile,
    util
  }
};
use ndarray::{Array, Ix2};

/* A struct representing a bomb.
 * While the lifetime of a bomb is always constant, the properties of one are affected by the stats
 * of the player who placed it.
 *
 * Position should be obvious.
 * Lifetime is the time (in frames) a bomb has until it detonates.
 * Piercing determines whether the bomb's blasts will go through breakable blocks rather than being
 * stopped by them.
 * Range determines how many blasts there are from the explosion in each cardinal direction. For
 * example, a bomb with 4 range will form this pattern assuming there is nothing to stop it, where B
 * represents a blast:
 *         B
 *         B
 *         B
 *         B
 * B B B B B B B B B
 *         B
 *         B
 *         B
 *         B
 * Even aside from this though, the blasts themselves depend on the immediate surroundings,
 * which means the world will have to handle blast calculation. On top of that, blast behavior
 * isn't as straight forward as you would imagine.
 *
 * When a bomb detonates, a blast instantly appears where it was. However, the rest of the
 * blasts take time to appear. Each frame after the explosion, a new blast is generated one tile
 * away from the previous blast, but must maintain its current direction. In other words,
 * the explosion is effectively "fanning out", but each blast shares the same lifetime. Lifetime
 * only starts ticking down once the last blasts have been generated.
 *
 * The world can definitely generate the blasts recursively, but the big issue is telling every
 * other blast when to start "dying". Maybe each "family" of blasts can all be kept in a single
 * struct, whose lifetime can start ticking down when it's finished exploding.
 */


/* A struct representing a bomb's blast.
 * Blasts are lethal to players and can break blocks that they touch. They can also potentially
 * destroy power-ups.
 *
 * Position should be obvious.
 * Like bombs, they have a lifetime, but naturally it should only be a short time.
 */



/* A trait representing a line of a blast's flames.
 * Considering that this is what makes up a blast, it's no surprise that this should also be lethal
 * to players.
 *
 * Since a flame is linear, it is marked by starting and ending points.
 * Direction determines the flame is spreading in.
 * Spread range determines the remaining range that the flame can spread in. This should be 0 if the
 * flame hits a wall, or if it has reached the end of its original range.
 */
pub trait Flame {
  /* Functionally ticks a flame by one frame.
   * The flame itself is dependent on the world deciding whether it has stopped or not, but if it
   * has, then it should not be able to spread anymore.
   */
  fn tick(&self, hit_wall: bool) -> Box<dyn Flame>;

  fn next_position(&self) -> (i8, i8);

  fn get_start(&self) -> &(i8, i8);

  fn get_end(&self) -> &(i8, i8);

  fn get_spread_range(&self) -> &i8;
}

pub struct FlameImpl {
  start: (i8, i8),
  end: (i8, i8),
  direction: Direction,
  spread_range: i8
}

impl FlameImpl {
  fn new(start: (i8, i8), end: (i8, i8), direction: Direction, spread_range: i8) -> FlameImpl {
    return FlameImpl {
      start: start,
      end: end,
      direction: direction,
      spread_range: spread_range
    }
  }
}

impl Flame for FlameImpl {
  fn tick(&self, hit_wall: bool) -> Box<dyn Flame> {
    let next_position: (i8, i8) = self.next_position();
    match hit_wall {
      true => return Box::new(
        FlameImpl {
          start: self.start,
          end: self.end,
          direction: self.direction,
          spread_range: 0,
        }
      ),
      false => {
        match self.spread_range {
          0 => return Box::new(
            FlameImpl {
              start: self.start,
              end: self.end,
              direction: self.direction,
              spread_range: 0
            }
          ),
          _ => return Box::new(
            FlameImpl {
              start: self.start,
              end: self.next_position(),
              direction: self.direction,
              spread_range: self.spread_range - 1
            }
          )
        }
      }
    }
  }

  fn next_position(&self) -> (i8, i8) {
    match self.direction {
      Direction::North => return (self.end.0, self.end.1 + 1),
      Direction::South => return (self.end.0, self.end.1 - 1),
      Direction::West => return (self.end.0 - 1, self.end.1),
      Direction::East => return (self.end.0 + 1, self.end.1),
      // We should NOT be getting intermediate directions for this.
      _ => return self.end
    }
  }

  fn get_start(&self) -> &(i8, i8) {
    return &self.start;
  }

  fn get_end(&self) -> &(i8, i8) {
    return &self.end;
  }

  fn get_spread_range(&self) -> &i8 {
    return &self.spread_range;
  }
}

pub trait Blast {
  fn tick(&self, hit_wall: Vec<bool>) -> Box<dyn Blast>;

  fn next_positions(&self) -> Vec<(i8, i8)>;

  fn get_center(&self) -> &(i8, i8);

  fn get_flames(&self) -> &Vec<Box<dyn Flame>>;

  fn get_lifetime(&self) -> &i8;
}

pub struct BlastImpl {
  center: (i8, i8),
  flames: Vec<Box<dyn Flame>>,
  spread_done: bool,
  lifetime: i8
}

impl BlastImpl {
  pub fn new(
    center: (i8, i8), range: i8,
    up_free: bool, down_free: bool, left_free: bool, right_free: bool
  ) -> BlastImpl {
    let up_point: (i8, i8) = (center.0, center.1 + 1);
    let down_point: (i8, i8) = (center.0, center.1 - 1);
    let left_point: (i8, i8) = (center.0 - 1, center.1);
    let right_point: (i8, i8) = (center.0 + 1, center.1);

    let flames: Vec<Box<dyn Flame>> = vec!();

    // We should only make blast flames in each cardinal direction if there's space for them.
    if up_free {
      flames.push(Box::new(FlameImpl::new(up_point, up_point, Direction::North, range)))
    }
    if down_free {
      flames.push(Box::new(FlameImpl::new(down_point, down_point, Direction::South, range)))
    }
    if left_free {
      flames.push(Box::new(FlameImpl::new(left_point, left_point, Direction::West, range)))
    }
    if right_free {
      flames.push(Box::new(FlameImpl::new(right_point, right_point, Direction::East, range)))
    }
 
    return BlastImpl {
      center: center,
      flames: flames,
      spread_done: false,
      lifetime: 60
    }
  }

  // Calculates this blast's flames on the next tick.
  fn calc_flames(&self, hit_wall: Vec<bool>) -> Vec<Box<dyn Flame>> {
    let flames: Vec<Box<dyn Flame>> = vec!();
    for i in 0..self.flames.len() {
      flames.push(self.flames[i].tick(hit_wall[i]));
    }
    return flames;
  }

  /* Calculates this blast's lifetime on the next tick. Additionally, this also determines whether
   * the spread of the blast has been completed, after which its lifetime can start ticking down.
   */
  fn calc_lifetime(&self) -> i8 {
    match self.spread_done {
      true => return self.lifetime - 1,
      false => return self.lifetime
    }
  }

  fn calc_spread_done(&self) -> bool {
    match self.spread_done {
      true => return true,
      false => {
        let all_done: bool = true;
        for flame in self.flames {
          all_done = all_done && flame.get_spread_range() == &0;
        }
        return all_done
      }
    }
  }
}

impl Blast for BlastImpl {
  fn tick(&self, hit_wall: Vec<bool>) -> Box<dyn Blast> {
    // We want to calculate lifetime first.
    let new_lifetime: i8 = self.calc_lifetime();

    // I don't think the order of the two steps below matter, but I could be wrong.
    let new_spread_done: bool = self.calc_spread_done();
    let new_flames: Vec<Box<dyn Flame>> = self.calc_flames(hit_wall);

    return Box::new(
      BlastImpl {
        center: self.center,
        flames: new_flames,
        spread_done: self.spread_done,
        lifetime: new_lifetime
      }
    )
  }

  fn next_positions(&self) -> Vec<(i8, i8)> {
    let next_positions: Vec<(i8, i8)> = vec!();
    for flame in self.flames {
      next_positions.push(flame.next_position());
    }
    return next_positions;
  }

  fn get_center(&self) -> &(i8, i8) {
    return &self.center;
  }

  fn get_flames(&self) -> &Vec<Box<dyn Flame>> {
    return &self.flames;
  }

  fn get_lifetime(&self) -> &i8 {
    return &self.lifetime;
  }
}

pub trait Bomb {
  fn tick(&self) -> Box<dyn Bomb>;

  fn can_detonate(&self) -> bool;

  fn get_position(&self) -> &(i8, i8);

  fn get_lifetime(&self) -> &i16;

  fn get_piercing(&self) -> &bool;

  fn get_range(&self) -> &i8;
}


pub struct BombImpl {
  position: (i8, i8),
  lifetime: i16,
  piercing: bool,
  range: i8
}

impl BombImpl {
  fn new(position: (i8, i8), piercing: bool, range: i8) -> BombImpl {
    return BombImpl {
      position: position,
      lifetime: 300,
      piercing: piercing,
      range: range
    }
  } 
}

impl Bomb for BombImpl {
  fn tick(&self) -> Box<dyn Bomb> {
    return Box::new(
      BombImpl {
        position: self.position,
        lifetime: self.lifetime - 1,
        piercing: self.piercing,
        range: self.range
      }
    )
  }

  fn can_detonate(&self) -> bool {
    return self.lifetime == 0
  }

  fn get_position(&self) -> &(i8, i8) {
    return &self.position;
  }

  fn get_lifetime(&self) -> &i16 {
    return &self.lifetime;
  }

  fn get_piercing(&self) -> &bool {
    return &self.piercing;
  }

  fn get_range(&self) -> &i8 {
    return &self.range
  }
}
