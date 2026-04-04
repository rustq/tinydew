use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    Farm,
    EastPath,
    Square,
    SouthRiver,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::Farm => write!(f, "Farm"),
            Region::EastPath => write!(f, "EastPath"),
            Region::Square => write!(f, "Square"),
            Region::SouthRiver => write!(f, "SouthRiver"),
        }
    }
}

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
pub enum TileType {
    Boundary,
    Grass,
    Soil,
    Crop(CropType, bool),
    House,
    PathEast,
    PathFarm,
    PathSquare,
    PathSouthRiver,
    PathSouthRiverGate,
    Mushroom,
    Fountain,
    River,
    RiverBubble,
    Wonder,
    Flower(FlowerState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowerState {
    pub mature: bool,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Boundary
            | TileType::House
            | TileType::Mushroom
            | TileType::Fountain
            | TileType::River
            | TileType::RiverBubble
            | TileType::Wonder => false,
            TileType::Crop(_, mature) => !(*mature),
            TileType::Flower(state) => !state.mature,
            _ => true,
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            TileType::Boundary => "🌳",
            TileType::Grass => "🌿",
            TileType::Soil => "🍃",
            TileType::Crop(crop, mature) => {
                if *mature {
                    crop.produce_emoji()
                } else {
                    "🌱"
                }
            }
            TileType::House => "🏠",
            TileType::PathEast
            | TileType::PathFarm
            | TileType::PathSquare
            | TileType::PathSouthRiver
            | TileType::PathSouthRiverGate => "🌿",
            TileType::Mushroom => "🍄",
            TileType::Fountain => "⛲",
            TileType::River => "🌊",
            TileType::RiverBubble => "🫧",
            TileType::Wonder => "🦋",
            TileType::Flower(state) => {
                if state.mature {
                    "🌺"
                } else {
                    "🌱"
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Carrot,
    Strawberry,
    Cauliflower,
    Flower,
}

impl CropType {
    pub fn maturity_days(&self) -> u32 {
        match self {
            CropType::Carrot => 3,
            CropType::Strawberry => 4,
            CropType::Cauliflower => 5,
            CropType::Flower => 2,
        }
    }

    pub fn produce_emoji(&self) -> &'static str {
        match self {
            CropType::Carrot => "🥕",
            CropType::Strawberry => "🍓",
            CropType::Cauliflower => "🥦",
            CropType::Flower => "🌺",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}

impl Weather {
    pub fn emoji(&self, is_night: bool) -> &'static str {
        if is_night {
            "🌙"
        } else {
            match self {
                Weather::Sunny => "☀️",
                Weather::Cloudy => "⛅",
                Weather::Rainy => "🌧",
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
}
