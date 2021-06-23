use crate::common::direction::Direction;

/* A struct representing a player.
 * Although their movement will probably be tied to a grid system at the moment, this is subject to
 * change. Regardless, these guys are the core of the game, as they are the only entities capable of
 * placing bombs. Since they all die in one hit, there is no need to keep track of HP.
 *
 * All a player needs is a position, but it's possible they'll require a function callback as a
 * field when controllers are introduced.
 */
#[derive(Copy, Clone)]
pub struct Player {
  speed: f32,
  position: (f32, f32),
  direction: Direction
}

impl PartialEq for Player {
  fn eq(&self, other: &Self) -> bool {
    return self.speed == other.speed
        && self.position == other.position
        && self.direction == other.direction;
  }
}

impl Eq for Player {}

impl Player {
  pub fn new (position: (f32, f32), direction: Direction) -> Player {
    return Player {
      speed: 0.1,
      position: position,
      direction: direction
    }
  }

  pub fn next_position(&self) -> (f32, f32){ 
    match self.direction {
      Direction::North => return (self.position.0, self.position.1 + self.speed),
      Direction::South => return (self.position.0, self.position.1 - self.speed),
      Direction::West => return (self.position.0 - self.speed, self.position.1),
      Direction::East => return (self.position.0 + self.speed, self.position.1),
      intermediate => {
        let linear_speed: f32 = self.speed / (2 as f32).sqrt();
        match intermediate {
          Direction::Northwest => {
            return (self.position.0 - linear_speed, self.position.1 + linear_speed);
          },
          Direction::Northeast => {
            return (self.position.0 + linear_speed, self.position.1 + linear_speed);
          },
          Direction::Southwest => {
            return (self.position.0 - linear_speed, self.position.1 - linear_speed);
          },
          Direction::Southeast => {
            return (self.position.0 - linear_speed, self.position.1 + linear_speed);
          }
        }
      }
    }
  }

  pub fn set_position(&self, position: (f32, f32)) -> Player {
    return Player {
      speed: self.speed,
      position: position,
      direction: self.direction
    }
  }

  pub fn set_next_position(&self) -> Player {
    return Player {
      speed: self.speed,
      position: self.next_position(),
      direction: self.direction
    }
  }

  pub fn set_direction(&self, direction: Direction) -> Player {
    return Player {
      speed: self.speed,
      position: self.position,
      direction: direction
    }
  }
}
