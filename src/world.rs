use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum TileType {
    Boundary,
    Grass,
    Soil,
    Crop(CropType, u8),
    House,
    PathEast,
    PathFarm,
    Player,
    Mushroom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropType {
    Carrot,
    Strawberry,
    Cauliflower,
    Rhubarb,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Boundary => false,
            TileType::Grass
            | TileType::Soil
            | TileType::Crop(_, _)
            | TileType::House
            | TileType::PathEast
            | TileType::PathFarm
            | TileType::Player
            | TileType::Mushroom => true,
        }
    }

    pub fn is_transition(&self) -> bool {
        matches!(self, TileType::PathEast | TileType::PathFarm)
    }

    pub fn to_emoji(&self) -> &'static str {
        match self {
            TileType::Boundary => "🌳",
            TileType::Grass => "🌿",
            TileType::Soil => "▪️",
            TileType::Crop(crop, stage) => {
                if *stage > 0 {
                    match crop {
                        CropType::Carrot => "🥕",
                        CropType::Strawberry => "🍓",
                        CropType::Cauliflower => "🥦",
                        CropType::Rhubarb => "🌺",
                    }
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
        write!(f, "{}", self.to_emoji())
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
