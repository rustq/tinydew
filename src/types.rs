use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    Farm,
    EastPath,
    Square,
    SouthRiver,
}

impl Region {
    pub const ALL: [Region; 4] = [
        Region::Farm,
        Region::EastPath,
        Region::Square,
        Region::SouthRiver,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
}

impl Weather {
    pub fn icon(&self, is_night: bool) -> &str {
        if is_night {
            "\u{1f319}" // 🌙
        } else {
            match self {
                Weather::Sunny => "\u{2600}\u{fe0f}", // ☀️
                Weather::Cloudy => "\u{26c5}",         // ⛅
                Weather::Rainy => "\u{1f327}",         // 🌧
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Carrot,
    Strawberry,
    Cauliflower,
}

impl CropType {
    pub fn emoji(&self) -> &str {
        match self {
            CropType::Carrot => "\u{1f955}",     // 🥕
            CropType::Strawberry => "\u{1f353}",  // 🍓
            CropType::Cauliflower => "\u{1f966}", // 🥦
        }
    }

    pub fn name(&self) -> &str {
        match self {
            CropType::Carrot => "carrot",
            CropType::Strawberry => "strawberry",
            CropType::Cauliflower => "cauliflower",
        }
    }

    pub fn sell_price(&self) -> i32 {
        match self {
            CropType::Carrot => 15,
            CropType::Strawberry => 20,
            CropType::Cauliflower => 25,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Boundary,
    Grass,
    Soil,
    Plant {
        crop: CropType,
        days_grown: u8,
        watered: bool,
    },
    House,
    PathEast,
    PathFarm,
    PathSquare,
    PathSouthRiver,
    PathSouthRiverGate,
    Flower,
    Mushroom,
    Fountain,
    River,
    RiverBubble,
    Wonder,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Grass
            | TileType::Soil
            | TileType::PathEast
            | TileType::PathFarm
            | TileType::PathSquare
            | TileType::PathSouthRiver
            | TileType::PathSouthRiverGate => true,
            TileType::Plant { days_grown, .. } => *days_grown < 1,
            _ => false,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            TileType::Boundary => "\u{1f333}",  // 🌳
            TileType::Grass => "\u{1f33f}",      // 🌿
            TileType::Soil => "\u{1f343}",       // 🍃
            TileType::Plant { days_grown, crop, .. } => {
                if *days_grown >= 1 {
                    crop.emoji()
                } else {
                    "\u{1f331}" // 🌱
                }
            }
            TileType::House => "\u{1f3e0}",      // 🏠
            TileType::PathEast
            | TileType::PathFarm
            | TileType::PathSquare
            | TileType::PathSouthRiver
            | TileType::PathSouthRiverGate => "\u{1f33f}", // 🌿
            TileType::Flower => "\u{1f33a}",     // 🌺
            TileType::Mushroom => "\u{1f344}",   // 🍄
            TileType::Fountain => "\u{26f2}",    // ⛲
            TileType::River => "\u{1f30a}",      // 🌊
            TileType::RiverBubble => "\u{1fab7}", // 🫧
            TileType::Wonder => "\u{1f98b}",     // 🦋
        }
    }

    pub fn is_fishable(&self) -> bool {
        matches!(self, TileType::River | TileType::RiverBubble)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub region: Region,
    pub direction: Direction,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub seeds: u32,
    pub carrots: u32,
    pub strawberries: u32,
    pub cauliflowers: u32,
    pub mushrooms: u32,
    pub flowers: u32,
    pub common_fish: u32,
    pub rare_fish: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegionMaps {
    pub farm: Vec<Vec<TileType>>,
    pub east_path: Vec<Vec<TileType>>,
    pub square: Vec<Vec<TileType>>,
    pub south_river: Vec<Vec<TileType>>,
}

impl RegionMaps {
    pub fn get(&self, region: &Region) -> &Vec<Vec<TileType>> {
        match region {
            Region::Farm => &self.farm,
            Region::EastPath => &self.east_path,
            Region::Square => &self.square,
            Region::SouthRiver => &self.south_river,
        }
    }

    pub fn get_mut(&mut self, region: &Region) -> &mut Vec<Vec<TileType>> {
        match region {
            Region::Farm => &mut self.farm,
            Region::EastPath => &mut self.east_path,
            Region::Square => &mut self.square,
            Region::SouthRiver => &mut self.south_river,
        }
    }
}

pub fn parse_direction(s: &str) -> Option<Direction> {
    match s {
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        "left" => Some(Direction::Left),
        "right" => Some(Direction::Right),
        _ => None,
    }
}

pub fn target_pos(
    x: usize,
    y: usize,
    dir: Direction,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    match dir {
        Direction::Up => {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        }
        Direction::Down => {
            if y + 1 < height {
                Some((x, y + 1))
            } else {
                None
            }
        }
        Direction::Left => {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        }
        Direction::Right => {
            if x + 1 < width {
                Some((x + 1, y))
            } else {
                None
            }
        }
    }
}

pub fn format_time(time_minutes: u32) -> String {
    let m = time_minutes % 1440;
    format!("{:02}:{:02}", m / 60, m % 60)
}

pub fn is_night(time_minutes: u32) -> bool {
    let m = time_minutes % 1440;
    m >= 1080 || m < 360
}
