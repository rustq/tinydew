use std::collections::HashMap;
use crate::farming::{CropType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Seed(CropType),
    Crop(CropType),
    Wood,
    Stone,
    Fish,
}

impl ItemType {
    pub fn to_emoji(&self) -> &'static str {
        match self {
            ItemType::Seed(_) => "🌾",
            ItemType::Crop(crop_type) => match crop_type {
                CropType::Strawberry => "🍓",
                CropType::Corn => "🌽",
                CropType::Tomato => "🍅",
                CropType::Pumpkin => "🎃",
                CropType::Carrot => "🥕",
                CropType::Eggplant => "🍆",
                CropType::Blueberry => "🫐",
            },
            ItemType::Wood => "🪵",
            ItemType::Stone => "🪨",
            ItemType::Fish => "🐟",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    items: HashMap<ItemType, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        let mut items = HashMap::new();
        items.insert(ItemType::Seed(CropType::Strawberry), 10);
        items.insert(ItemType::Seed(CropType::Corn), 10);
        items.insert(ItemType::Seed(CropType::Carrot), 10);
        Self { items }
    }

    pub fn add(&mut self, item_type: ItemType, quantity: u32) {
        *self.items.entry(item_type).or_insert(0) += quantity;
    }

    pub fn remove(&mut self, item_type: ItemType, quantity: u32) -> bool {
        if let Some(count) = self.items.get_mut(&item_type) {
            if *count >= quantity {
                *count -= quantity;
                if *count == 0 {
                    self.items.remove(&item_type);
                }
                return true;
            }
        }
        false
    }

    pub fn count(&self, item_type: ItemType) -> u32 {
        self.items.get(&item_type).copied().unwrap_or(0)
    }

    pub fn items(&self) -> &HashMap<ItemType, u32> {
        &self.items
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ItemType, &u32)> {
        self.items.iter()
    }
}
