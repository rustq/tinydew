use crate::map::RegionMap;
use crate::types::{FlowerState, Region, TileType};
use rand::{Rng, SeedableRng};

pub fn get_valid_spawn_tiles(map: &RegionMap) -> Vec<(usize, usize)> {
    let mut valid = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(tile) = map.get(x, y) {
                if matches!(tile, TileType::Grass)
                    && !(map.region == Region::Square && (x == 4 && y == 2))
                    && !(map.region == Region::Farm && (x == 2 && y == 2))
                {
                    valid.push((x, y));
                }
            }
        }
    }
    valid
}

pub fn spawn_daily_flowers(
    maps: &mut HashMap<Region, RegionMap>,
    day: u32,
    seed: u64,
) -> Vec<(Region, usize, usize)> {
    let mut spawned = Vec::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed.wrapping_mul(day as u64));

    for (&region, map) in maps.iter_mut() {
        let has_flower = (0..map.height)
            .any(|y| (0..map.width).any(|x| matches!(map.get(x, y), Some(TileType::Flower(_)))));

        if !has_flower {
            let valid_tiles = get_valid_spawn_tiles(map);
            if !valid_tiles.is_empty() {
                let idx = rng.gen_range(0..valid_tiles.len());
                let (x, y) = valid_tiles[idx];
                map.set(x, y, TileType::Flower(FlowerState { mature: false }));
                spawned.push((region, x, y));
            }
        }
    }

    spawned
}

pub fn spawn_daily_mushrooms(
    maps: &mut HashMap<Region, RegionMap>,
    day: u32,
    seed: u64,
) -> Vec<(Region, usize, usize)> {
    let mut spawned = Vec::new();
    let mut rng =
        rand::rngs::StdRng::seed_from_u64(seed.wrapping_mul(day as u64).wrapping_add(1000));

    for (&region, map) in maps.iter_mut() {
        let has_mushroom = (0..map.height)
            .any(|y| (0..map.width).any(|x| matches!(map.get(x, y), Some(TileType::Mushroom))));

        if !has_mushroom && region != Region::SouthRiver {
            let valid_tiles = get_valid_spawn_tiles(map);
            if !valid_tiles.is_empty() {
                let idx = rng.gen_range(0..valid_tiles.len());
                let (x, y) = valid_tiles[idx];
                map.set(x, y, TileType::Mushroom);
                spawned.push((region, x, y));
            }
        }
    }

    spawned
}

use std::collections::HashMap;
