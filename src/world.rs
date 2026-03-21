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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TileType {
    Boundary,
    Grass,
    Soil,
    Crop(CropType, CropState),
    House,
    PathEast,
    PathFarm,
    PathSquare,
    PathSouthRiver,
    PathSouthRiverGate,
    Player,
    Mushroom,
    Fountain,
    Slide,
    River,
    RiverBubble,
    Wonder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FishType {
    Common,
    Rare,
}

impl FishType {
    pub fn emoji(&self) -> &'static str {
        match self {
            FishType::Common => "🐟",
            FishType::Rare => "🐠",
        }
    }

    pub fn sell_price(&self) -> u32 {
        match self {
            FishType::Common => 80,
            FishType::Rare => 180,
        }
    }

    pub fn all() -> [FishType; 2] {
        [FishType::Common, FishType::Rare]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}

impl Weather {
    pub fn icon(&self) -> &'static str {
        match self {
            Weather::Sunny => "☀️",
            Weather::Cloudy => "⛅",
            Weather::Rainy => "🌧",
        }
    }
}

impl ForageType {
    pub fn emoji(&self) -> &'static str {
        match self {
            ForageType::Mushroom => "🍄\u{200d}🟫",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ForageType::Mushroom => "Mushroom",
        }
    }

    pub fn sell_price(&self) -> u32 {
        match self {
            ForageType::Mushroom => 25,
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
            TileType::Crop(crop, state) => !state.is_mature(*crop),
            TileType::Fountain => false,
            TileType::Slide => false,
            TileType::Mushroom => false,
            TileType::River => false,
            TileType::RiverBubble => false,
            TileType::Wonder => false,
            TileType::Grass
            | TileType::Soil
            | TileType::PathEast
            | TileType::PathFarm
            | TileType::PathSquare
            | TileType::PathSouthRiver
            | TileType::PathSouthRiverGate
            | TileType::Player => true,
            TileType::House => false,
        }
    }

    pub fn is_transition(&self) -> bool {
        matches!(
            self,
            TileType::PathEast
                | TileType::PathFarm
                | TileType::PathSquare
                | TileType::PathSouthRiver
                | TileType::PathSouthRiverGate
        )
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            TileType::Boundary => "🌳",
            TileType::Grass => "🌿",
            TileType::Soil => "🍃",
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
            TileType::PathSquare => "🌿",
            TileType::PathSouthRiver => "🌿",
            TileType::PathSouthRiverGate => "🌿",
            TileType::Player => "🧑",
            TileType::Mushroom => "🍄",
            TileType::Fountain => "⛲",
            TileType::Slide => "🛝",
            TileType::River => "🌊",
            TileType::RiverBubble => "🫧",
            TileType::Wonder => "🦋",
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
            TileType::Crop(
                CropType::Rhubarb,
                CropState {
                    days_grown: 16,
                    watered_today: false,
                },
            ),
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
            TileType::PathSquare,
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
            TileType::PathSouthRiver,
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

pub fn create_square_map() -> Map {
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
        ],
        vec![
            TileType::Boundary,
            TileType::Crop(
                CropType::Rhubarb,
                CropState {
                    days_grown: 16,
                    watered_today: false,
                },
            ),
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
            TileType::Fountain,
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
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::PathSquare,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
            TileType::Boundary,
        ],
    ]
}

pub fn create_south_river_map() -> Map {
    vec![
        vec![
            TileType::Boundary,
            TileType::Boundary,
            TileType::PathSouthRiverGate,
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
            TileType::Grass,
            TileType::Grass,
            TileType::Boundary,
        ],
        vec![
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
        ],
        vec![
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
            TileType::River,
        ],
    ]
}

pub const FARM_WIDTH: usize = 8;
pub const FARM_HEIGHT: usize = 8;
pub const EAST_PATH_WIDTH: usize = 11;
pub const EAST_PATH_HEIGHT: usize = 4;
pub const SQUARE_WIDTH: usize = 9;
pub const SQUARE_HEIGHT: usize = 5;
pub const SOUTH_RIVER_WIDTH: usize = 13;
pub const SOUTH_RIVER_HEIGHT: usize = 4;
