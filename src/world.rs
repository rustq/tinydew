use serde::{Deserialize, Serialize};
use std::fmt;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TileType {
    Boundary,
    Grass,
    Soil,
    Crop(CropType, CropState),
    House,
    PathEast,
    PathFarm,
    Player,
    Mushroom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CropState {
    pub days_grown: u8,
    pub watered_today: bool,
}

impl CropState {
    pub fn new() -> Self {
        Self {
            days_grown: 0,
            watered_today: false,
        }
    }

    pub fn is_mature(&self, crop_type: CropType) -> bool {
        let days_needed = crop_type.growth_days();
        self.days_grown >= days_needed
    }
}

impl Default for CropState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CropType {
    Carrot,
    Strawberry,
    Cauliflower,
    Rhubarb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForageType {
    Mushroom,
}

impl ForageType {
    pub fn emoji(&self) -> &'static str {
        match self {
            ForageType::Mushroom => "🍄",
        }
    }
}

impl CropType {
    pub fn growth_days(&self) -> u8 {
        match self {
            CropType::Carrot => 4,
            CropType::Strawberry => 8,
            CropType::Cauliflower => 12,
            CropType::Rhubarb => 16,
        }
    }

    pub fn produce_emoji(&self) -> &'static str {
        match self {
            CropType::Carrot => "🥕",
            CropType::Strawberry => "🍓",
            CropType::Cauliflower => "🥦",
            CropType::Rhubarb => "🌺",
        }
    }

    pub fn seed_name(&self) -> &'static str {
        match self {
            CropType::Carrot => "Carrot",
            CropType::Strawberry => "Strawberry",
            CropType::Cauliflower => "Cauliflower",
            CropType::Rhubarb => "Rhubarb",
        }
    }

    pub fn seed_price(&self) -> u32 {
        match self {
            CropType::Carrot => 10,
            CropType::Strawberry => 20,
            CropType::Cauliflower => 30,
            CropType::Rhubarb => 40,
        }
    }

    pub fn produce_price(&self) -> u32 {
        match self {
            CropType::Carrot => 20,
            CropType::Strawberry => 40,
            CropType::Cauliflower => 60,
            CropType::Rhubarb => 80,
        }
    }

    pub fn all() -> [CropType; 4] {
        [
            CropType::Carrot,
            CropType::Strawberry,
            CropType::Cauliflower,
            CropType::Rhubarb,
        ]
    }
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Boundary => false,
            TileType::Crop(_, _) => false,
            TileType::Grass
            | TileType::Soil
            | TileType::House
            | TileType::PathEast
            | TileType::PathFarm
            | TileType::Player => true,
            TileType::Mushroom => false,
        }
    }

    pub fn is_transition(&self) -> bool {
        matches!(self, TileType::PathEast | TileType::PathFarm)
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            TileType::Boundary => "🌳",
            TileType::Grass => "🌿",
            TileType::Soil => "▪️",
            TileType::Crop(crop, state) => {
                if state.is_mature(*crop) {
                    crop.produce_emoji()
                } else {
                    "🌱"
                }
            }
            TileType::House => "🏠",
            TileType::PathEast => "🌿",
            TileType::PathFarm => "🌿",
            TileType::Player => "🧑",
            TileType::Mushroom => "🍄",
        }
    }
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.emoji())
    }
}

pub type Map = Vec<Vec<TileType>>;

pub fn create_farm_map() -> Map {
    vec![
        vec![
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::House,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Crop(CropType::Rhubarb, CropState::new()),
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::PathEast,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
        ],
    ]
}

pub fn create_east_path_map() -> Map {
    vec![
        vec![
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::PathFarm,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Grass,
            TileType::Mushroom,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
        ],
    ]
}

pub const FARM_WIDTH: usize = 8;
pub const FARM_HEIGHT: usize = 8;
pub const EAST_PATH_WIDTH: usize = 11;
pub const EAST_PATH_HEIGHT: usize = 4;
