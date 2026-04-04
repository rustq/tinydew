use serde::{Deserialize, Serialize};

use crate::tile::CropType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FishType {
    Common,
    Rare,
}

impl FishType {
    #[allow(dead_code)]
    pub fn emoji(self) -> &'static str {
        match self {
            FishType::Common => "🐟",
            FishType::Rare => "🐠",
        }
    }

    pub fn sell_price(self) -> i32 {
        match self {
            FishType::Common => 20,
            FishType::Rare => 80,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub seeds: u32,
    pub carrots: u32,
    pub strawberries: u32,
    pub cauliflowers: u32,
    pub flowers: u32,
    pub mushrooms: u32,
    pub common_fish: u32,
    pub rare_fish: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            seeds: 5,
            carrots: 0,
            strawberries: 0,
            cauliflowers: 0,
            flowers: 0,
            mushrooms: 0,
            common_fish: 0,
            rare_fish: 0,
        }
    }

    pub fn add_produce(&mut self, crop_type: CropType) {
        match crop_type {
            CropType::Carrot => self.carrots += 1,
            CropType::Strawberry => self.strawberries += 1,
            CropType::Cauliflower => self.cauliflowers += 1,
            CropType::Flower => self.flowers += 1,
        }
    }

    pub fn add_fish(&mut self, fish_type: FishType) {
        match fish_type {
            FishType::Common => self.common_fish += 1,
            FishType::Rare => self.rare_fish += 1,
        }
    }

    /// Returns list of (emoji, count) for non-empty inventory items (excluding seeds).
    pub fn display_items(&self) -> Vec<(&str, u32)> {
        let mut items = Vec::new();
        if self.carrots > 0 {
            items.push(("🥕", self.carrots));
        }
        if self.strawberries > 0 {
            items.push(("🍓", self.strawberries));
        }
        if self.cauliflowers > 0 {
            items.push(("🥦", self.cauliflowers));
        }
        if self.flowers > 0 {
            items.push(("🌺", self.flowers));
        }
        if self.mushrooms > 0 {
            items.push(("🍄", self.mushrooms));
        }
        if self.common_fish > 0 {
            items.push(("🐟", self.common_fish));
        }
        if self.rare_fish > 0 {
            items.push(("🐠", self.rare_fish));
        }
        items
    }

    /// Try to sell an item by emoji. Returns (price, success).
    pub fn try_sell(&mut self, emoji: &str) -> Result<i32, String> {
        match emoji {
            "🥕" => {
                if self.carrots > 0 {
                    self.carrots -= 1;
                    Ok(CropType::Carrot.sell_price())
                } else {
                    Err("No carrots to sell.".to_string())
                }
            }
            "🍓" => {
                if self.strawberries > 0 {
                    self.strawberries -= 1;
                    Ok(CropType::Strawberry.sell_price())
                } else {
                    Err("No strawberries to sell.".to_string())
                }
            }
            "🥦" => {
                if self.cauliflowers > 0 {
                    self.cauliflowers -= 1;
                    Ok(CropType::Cauliflower.sell_price())
                } else {
                    Err("No cauliflowers to sell.".to_string())
                }
            }
            "🌺" => {
                if self.flowers > 0 {
                    self.flowers -= 1;
                    Ok(CropType::Flower.sell_price())
                } else {
                    Err("No flowers to sell.".to_string())
                }
            }
            "🍄" => {
                if self.mushrooms > 0 {
                    self.mushrooms -= 1;
                    Ok(25)
                } else {
                    Err("No mushrooms to sell.".to_string())
                }
            }
            "🐟" => {
                if self.common_fish > 0 {
                    self.common_fish -= 1;
                    Ok(FishType::Common.sell_price())
                } else {
                    Err("No common fish to sell.".to_string())
                }
            }
            "🐠" => {
                if self.rare_fish > 0 {
                    self.rare_fish -= 1;
                    Ok(FishType::Rare.sell_price())
                } else {
                    Err("No rare fish to sell.".to_string())
                }
            }
            _ => Err(format!("Unknown item: {emoji}")),
        }
    }
}

pub const SEED_PRICE: i32 = 20;
