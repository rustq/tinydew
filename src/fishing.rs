use crate::map::RegionMap;
use crate::types::{FishType, Region, TileType};
use rand::{Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct FishResult {
    pub caught: Option<FishType>,
    pub message: String,
}

pub fn roll_fish(seed: u64) -> Option<FishType> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let roll = rng.gen_range(0..100);

    if roll < 60 {
        Some(FishType::Common)
    } else if roll < 80 {
        Some(FishType::Rare)
    } else {
        None
    }
}

pub fn try_fish(map: &mut RegionMap, x: usize, y: usize, seed: u64) -> FishResult {
    if let Some(tile) = map.get(x, y) {
        if matches!(tile, TileType::River) {
            map.set(x, y, TileType::RiverBubble);

            if let Some(fish) = roll_fish(seed) {
                FishResult {
                    caught: Some(fish),
                    message: format!("You caught a {}!", fish.emoji()),
                }
            } else {
                FishResult {
                    caught: None,
                    message: "No bite...".to_string(),
                }
            }
        } else if matches!(tile, TileType::RiverBubble) {
            FishResult {
                caught: None,
                message: "The bubble pops but there's nothing here...".to_string(),
            }
        } else {
            FishResult {
                caught: None,
                message: "You can't fish here.".to_string(),
            }
        }
    } else {
        FishResult {
            caught: None,
            message: "You can't fish here.".to_string(),
        }
    }
}

pub fn reset_river_bubbles(maps: &mut HashMap<Region, RegionMap>) {
    for (_, map) in maps.iter_mut() {
        for y in 0..map.height {
            for x in 0..map.width {
                if matches!(map.get(x, y), Some(TileType::RiverBubble)) {
                    map.set(x, y, TileType::River);
                }
            }
        }
    }
}

use std::collections::HashMap;
