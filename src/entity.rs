use serde::{Deserialize, Serialize};

use crate::map::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub location: Location,
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 3,
            y: 3,
            location: Location::Farm,
            direction: Direction::Down,
        }
    }

    pub fn target_pos(&self, dir: Direction) -> (i32, i32) {
        let (dx, dy) = dir.delta();
        (self.x as i32 + dx, self.y as i32 + dy)
    }
}
