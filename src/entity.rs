use crate::types::{Direction, Region};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub region: Region,
    pub direction: Direction,
}

impl Player {
    pub fn new(x: usize, y: usize, region: Region) -> Self {
        Self {
            x,
            y,
            region,
            direction: Direction::Down,
        }
    }

    pub fn move_to(&mut self, x: usize, y: usize, region: Region, direction: Direction) {
        self.x = x;
        self.y = y;
        self.region = region;
        self.direction = direction;
    }
}
