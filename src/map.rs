use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Region {
    Farm,
    EastPath,
    Square,
    SouthRiver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Carrot,
    Strawberry,
    Cauliflower,
    Flower,
}

impl CropType {
    pub fn emoji(&self) -> &'static str {
        match self {
            CropType::Carrot => "🥕",
            CropType::Strawberry => "🍓",
            CropType::Cauliflower => "🥦",
            CropType::Flower => "🌺",
        }
    }

    pub fn maturity_days(&self) -> u32 {
        match self {
            CropType::Carrot => 3,
            CropType::Strawberry => 4,
            CropType::Cauliflower => 5,
            CropType::Flower => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Grass,
    Tree,
    House,
    PathEast,
    PathFarm,
    PathSquare,
    PathSouthRiver,
    PathSouthRiverGate,
    Fountain,
    Piano,
    River,
    RiverBubble,
    Soil,
    Crop {
        crop_type: CropType,
        days_grown: u32,
        watered_today: bool,
    },
    Mushroom,
    Wonder,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Tree => false,
            TileType::House => false,
            TileType::Fountain => false,
            TileType::Piano => false,
            TileType::River => false,
            TileType::RiverBubble => false,
            TileType::Mushroom => false,
            TileType::Wonder => false,
            TileType::Crop { crop_type, days_grown, .. } => {
                *days_grown < crop_type.maturity_days()
            }
            _ => true,
        }
    }

    pub fn is_mature_crop(&self) -> bool {
        match self {
            TileType::Crop { crop_type, days_grown, .. } => {
                *days_grown >= crop_type.maturity_days()
            }
            _ => false,
        }
    }

    pub fn is_path(&self) -> bool {
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
            TileType::Grass => "🌿",
            TileType::Tree => "🌳",
            TileType::House => "🏠",
            TileType::PathEast
            | TileType::PathFarm
            | TileType::PathSquare
            | TileType::PathSouthRiver
            | TileType::PathSouthRiverGate => "🌿",
            TileType::Fountain => "⛲",
            TileType::Piano => "🎹",
            TileType::River => "🌊",
            TileType::RiverBubble => "🫧",
            TileType::Soil => "🍃",
            TileType::Crop { crop_type, days_grown, .. } => {
                if *days_grown >= crop_type.maturity_days() {
                    crop_type.emoji()
                } else {
                    "🌱"
                }
            }
            TileType::Mushroom => "🍄",
            TileType::Wonder => "🦋",
        }
    }
}

pub type RegionMap = Vec<Vec<TileType>>;

pub fn region_dimensions(region: Region) -> (usize, usize) {
    match region {
        Region::Farm => (8, 8),
        Region::EastPath => (11, 4),
        Region::Square => (9, 5),
        Region::SouthRiver => (13, 4),
    }
}

pub fn create_farm() -> RegionMap {
    let (w, h) = (8, 8);
    let mut map = vec![vec![TileType::Grass; w]; h];
    // Boundary ring
    for x in 0..w {
        map[0][x] = TileType::Tree;
        map[h - 1][x] = TileType::Tree;
    }
    for y in 0..h {
        map[y][0] = TileType::Tree;
        map[y][w - 1] = TileType::Tree;
    }
    // Fixed tiles
    map[2][2] = TileType::House;
    map[2][4] = TileType::Piano;
    map[5][7] = TileType::PathEast;
    map
}

pub fn create_east_path() -> RegionMap {
    let (w, h) = (11, 4);
    let mut map = vec![vec![TileType::Grass; w]; h];
    // Top row mostly boundary
    for x in 0..w {
        map[0][x] = TileType::Tree;
    }
    // Bottom row mostly boundary
    for x in 0..w {
        map[h - 1][x] = TileType::Tree;
    }
    // Right edge boundary
    for y in 0..h {
        map[y][w - 1] = TileType::Tree;
    }
    // Fixed tiles (override boundaries)
    map[2][0] = TileType::PathFarm;
    map[0][5] = TileType::PathSquare;
    map[3][2] = TileType::PathSouthRiver;
    map[2][9] = TileType::Mushroom;
    map
}

pub fn create_square() -> RegionMap {
    let (w, h) = (9, 5);
    let mut map = vec![vec![TileType::Grass; w]; h];
    // Boundary ring
    for x in 0..w {
        map[0][x] = TileType::Tree;
        map[h - 1][x] = TileType::Tree;
    }
    for y in 0..h {
        map[y][0] = TileType::Tree;
        map[y][w - 1] = TileType::Tree;
    }
    // Fixed tiles
    map[2][4] = TileType::Fountain;
    map[1][1] = TileType::Crop {
        crop_type: CropType::Flower,
        days_grown: 2,
        watered_today: false,
    }; // Pre-planted mature flower
    map[4][4] = TileType::PathSquare;
    map
}

pub fn create_south_river() -> RegionMap {
    let (w, h) = (13, 4);
    let mut map = vec![vec![TileType::Grass; w]; h];
    // Gate
    map[0][2] = TileType::PathSouthRiverGate;
    // Top row boundaries except gate
    for x in 0..w {
        if x != 2 {
            map[0][x] = TileType::Tree;
        }
    }
    // River rows y=2..3
    for x in 0..w {
        map[2][x] = TileType::River;
        map[3][x] = TileType::River;
    }
    map
}
