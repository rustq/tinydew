use serde::{Deserialize, Serialize};
use crate::map::Region;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn from_str(s: &str) -> Option<Direction> {
        match s.to_lowercase().as_str() {
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "left" => Some(Direction::Left),
            "right" => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub x: usize,
    pub y: usize,
    pub location: Region,
    pub direction: Direction,
}

impl Entity {
    pub fn new(x: usize, y: usize, location: Region, direction: Direction) -> Self {
        Self { x, y, location, direction }
    }
}
