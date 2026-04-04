use serde::{Deserialize, Serialize};

use crate::tile::{CropData, CropType, TileType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Location {
    Farm,
    EastPath,
    Square,
    SouthRiver,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Farm => write!(f, "Farm"),
            Location::EastPath => write!(f, "EastPath"),
            Location::Square => write!(f, "Square"),
            Location::SouthRiver => write!(f, "SouthRiver"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionMap {
    pub location: Location,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
}

impl RegionMap {
    pub fn get(&self, x: usize, y: usize) -> Option<&TileType> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut TileType> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
}

fn fill_boundary_ring(tiles: &mut [Vec<TileType>], width: usize, height: usize) {
    for tile in &mut tiles[0] {
        *tile = TileType::Boundary;
    }
    for tile in &mut tiles[height - 1] {
        *tile = TileType::Boundary;
    }
    for row in tiles.iter_mut() {
        row[0] = TileType::Boundary;
        row[width - 1] = TileType::Boundary;
    }
}

pub fn create_farm() -> RegionMap {
    let width = 8;
    let height = 8;
    let mut tiles = vec![vec![TileType::Grass; width]; height];

    fill_boundary_ring(&mut tiles, width, height);

    // House at (2,2)
    tiles[2][2] = TileType::House;

    // PathEast at (7,5)
    tiles[5][7] = TileType::PathEast;

    RegionMap {
        location: Location::Farm,
        width,
        height,
        tiles,
    }
}

pub fn create_east_path() -> RegionMap {
    let width = 11;
    let height = 4;
    let mut tiles = vec![vec![TileType::Grass; width]; height];

    // Top row boundary
    for tile in &mut tiles[0] {
        *tile = TileType::Boundary;
    }
    // Bottom row boundary
    for tile in &mut tiles[height - 1] {
        *tile = TileType::Boundary;
    }
    // Right edge boundary
    for row in &mut tiles {
        row[width - 1] = TileType::Boundary;
    }

    // PathFarm at (0,2)
    tiles[2][0] = TileType::PathFarm;

    // PathSquare at (5,0)
    tiles[0][5] = TileType::PathSquare;

    // PathSouthRiver at (2,3)
    tiles[3][2] = TileType::PathSouthRiver;

    // Mushroom at (9,2)
    tiles[2][9] = TileType::Mushroom;

    RegionMap {
        location: Location::EastPath,
        width,
        height,
        tiles,
    }
}

pub fn create_square() -> RegionMap {
    let width = 9;
    let height = 5;
    let mut tiles = vec![vec![TileType::Grass; width]; height];

    fill_boundary_ring(&mut tiles, width, height);

    // Fountain at (4,2)
    tiles[2][4] = TileType::Fountain;

    // Pre-planted mature Flower at (1,1)
    tiles[1][1] = TileType::Crop(CropData {
        crop_type: CropType::Flower,
        days_grown: CropType::Flower.days_to_mature(),
        watered_today: false,
    });

    // PathSquare at (4,4)
    tiles[4][4] = TileType::PathSquare;

    RegionMap {
        location: Location::Square,
        width,
        height,
        tiles,
    }
}

pub fn create_south_river() -> RegionMap {
    let width = 13;
    let height = 4;
    let mut tiles = vec![vec![TileType::Grass; width]; height];

    // PathSouthRiverGate at (2,0)
    tiles[0][2] = TileType::PathSouthRiverGate;

    // Boundaries for row 0 (except the gate)
    for (x, tile) in tiles[0].iter_mut().enumerate() {
        if x != 2 {
            *tile = TileType::Boundary;
        }
    }

    // River occupies rows y=2..3 across all columns x=0..12
    for row in tiles.iter_mut().skip(2) {
        for tile in row.iter_mut() {
            *tile = TileType::River;
        }
    }

    RegionMap {
        location: Location::SouthRiver,
        width,
        height,
        tiles,
    }
}
