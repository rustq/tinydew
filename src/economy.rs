use crate::types::FishType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub seeds: u32,
    pub produce: HashMap<String, u32>,
    pub forage: HashMap<String, u32>,
    pub fish: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_produce(&mut self, emoji: &str, qty: u32) {
        *self.produce.entry(emoji.to_string()).or_insert(0) += qty;
    }

    pub fn remove_produce(&mut self, emoji: &str, qty: u32) -> bool {
        if let Some(count) = self.produce.get_mut(emoji) {
            if *count >= qty {
                *count -= qty;
                if *count == 0 {
                    self.produce.remove(emoji);
                }
                return true;
            }
        }
        false
    }

    pub fn add_forage(&mut self, emoji: &str, qty: u32) {
        *self.forage.entry(emoji.to_string()).or_insert(0) += qty;
    }

    pub fn remove_forage(&mut self, emoji: &str, qty: u32) -> bool {
        if let Some(count) = self.forage.get_mut(emoji) {
            if *count >= qty {
                *count -= qty;
                if *count == 0 {
                    self.forage.remove(emoji);
                }
                return true;
            }
        }
        false
    }

    pub fn add_fish(&mut self, fish_type: FishType, qty: u32) {
        let emoji = fish_type.emoji().to_string();
        *self.fish.entry(emoji).or_insert(0) += qty;
    }

    pub fn remove_fish(&mut self, emoji: &str, qty: u32) -> bool {
        if let Some(count) = self.fish.get_mut(emoji) {
            if *count >= qty {
                *count -= qty;
                if *count == 0 {
                    self.fish.remove(emoji);
                }
                return true;
            }
        }
        false
    }

    pub fn has_seeds(&self) -> bool {
        self.seeds > 0
    }

    pub fn use_seed(&mut self) -> bool {
        if self.seeds > 0 {
            self.seeds -= 1;
            true
        } else {
            false
        }
    }

    pub fn add_seeds(&mut self, qty: u32) {
        self.seeds += qty;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    pub seed_price: u32,
    pub produce_prices: HashMap<String, u32>,
    pub fish_prices: HashMap<String, u32>,
    pub mushroom_price: u32,
}

impl Shop {
    pub fn new() -> Self {
        let mut produce_prices = HashMap::new();
        produce_prices.insert("🥕".to_string(), 10);
        produce_prices.insert("🍓".to_string(), 15);
        produce_prices.insert("🥦".to_string(), 20);
        produce_prices.insert("🌺".to_string(), 5);

        let mut fish_prices = HashMap::new();
        fish_prices.insert("🐟".to_string(), 5);
        fish_prices.insert("🐠".to_string(), 25);

        Self {
            seed_price: 10,
            produce_prices,
            fish_prices,
            mushroom_price: 25,
        }
    }
}

impl Default for Shop {
    fn default() -> Self {
        Self::new()
    }
}
