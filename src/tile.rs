use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropType {
    Carrot,
    Strawberry,
    Cauliflower,
    Flower,
}

impl CropType {
    pub fn days_to_mature(self) -> u32 {
        match self {
            CropType::Carrot => 3,
            CropType::Strawberry => 4,
            CropType::Cauliflower => 5,
            CropType::Flower => 2,
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            CropType::Carrot => "🥕",
            CropType::Strawberry => "🍓",
            CropType::Cauliflower => "🥦",
            CropType::Flower => "🌺",
        }
    }

    pub fn sell_price(self) -> i32 {
        match self {
            CropType::Carrot => 35,
            CropType::Strawberry => 50,
            CropType::Cauliflower => 75,
            CropType::Flower => 20,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CropData {
    pub crop_type: CropType,
    pub days_grown: u32,
    pub watered_today: bool,
}

impl CropData {
    pub fn new(crop_type: CropType) -> Self {
        Self {
            crop_type,
            days_grown: 0,
            watered_today: false,
        }
    }

    pub fn is_mature(&self) -> bool {
        self.days_grown >= self.crop_type.days_to_mature()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Boundary,
    Grass,
    Soil,
    Crop(CropData),
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
}

impl TileType {
    pub fn emoji(&self) -> &str {
        match self {
            TileType::Boundary => "🌳",
            TileType::Grass => "🌿",
            TileType::Soil => "🍃",
            TileType::Crop(data) => {
                if data.is_mature() {
                    data.crop_type.emoji()
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
            TileType::Mushroom => "🍄",
            TileType::Fountain => "⛲",
            TileType::River => "🌊",
            TileType::RiverBubble => "🫧",
            TileType::Wonder => "🦋",
        }
    }

    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Boundary => false,
            TileType::Grass => true,
            TileType::Soil => true,
            TileType::Crop(data) => !data.is_mature(),
            TileType::House => false,
            TileType::PathEast => true,
            TileType::PathFarm => true,
            TileType::PathSquare => true,
            TileType::PathSouthRiver => true,
            TileType::PathSouthRiverGate => true,
            TileType::Mushroom => false,
            TileType::Fountain => false,
            TileType::River => false,
            TileType::RiverBubble => false,
            TileType::Wonder => false,
        }
    }

    pub fn is_clearable(&self) -> bool {
        matches!(self, TileType::Grass)
    }

    pub fn is_plantable(&self) -> bool {
        matches!(self, TileType::Soil)
    }

    pub fn is_fishable(&self) -> bool {
        matches!(self, TileType::River | TileType::RiverBubble)
    }
}
