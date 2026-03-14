#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainType {
    Grass,
    Weeds,
    Soil,
    Water,
    Stone,
    Wood,
    Tree,
    Farmhouse,
}

use crate::farming::Crop;

#[derive(Debug, Clone)]
pub struct Tile {
    pub terrain: TerrainType,
    pub last_watered_day: Option<u32>,
    pub crop: Option<Crop>,
    pub watered: bool,
}

impl Tile {
    pub fn new(terrain: TerrainType) -> Self {
        Self {
            terrain,
            last_watered_day: None,
            crop: None,
            watered: false,
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(
            self.terrain,
            TerrainType::Grass | TerrainType::Weeds | TerrainType::Soil
        )
    }

    pub fn to_emoji(&self) -> &'static str {
        if let Some(crop) = &self.crop {
            return crop.to_emoji();
        }
        
        match self.terrain {
            TerrainType::Grass => "🌿",
            TerrainType::Weeds => "🌾",
            TerrainType::Soil => "🌱",
            TerrainType::Water => "🌊",
            TerrainType::Stone => "🪨",
            TerrainType::Wood => "🪵",
            TerrainType::Tree => "🌳",
            TerrainType::Farmhouse => "🏚",
        }
    }

    pub fn to_ascii(&self) -> char {
        match self.terrain {
            TerrainType::Grass => '.',
            TerrainType::Weeds => 'w',
            TerrainType::Soil => ',',
            TerrainType::Water => '~',
            TerrainType::Stone => '#',
            TerrainType::Wood => '=',
            TerrainType::Tree => 'T',
            TerrainType::Farmhouse => 'H',
        }
    }
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![
            vec![Tile::new(TerrainType::Grass); width];
            height
        ];
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < self.width && y < self.height {
            Some(&mut self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub x: usize,
    pub y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct World {
    map: Map,
    player: Player,
}

impl World {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut world = Self {
            map: Map::new(30, 20),
            player: Player::new(5, 10),
        };

        world.setup_farm_map();
        Ok(world)
    }

    fn setup_farm_map(&mut self) {
        // Add river on the right side
        for y in 0..20 {
            self.map.set_tile(25, y, Tile::new(TerrainType::Water));
        }

        // Add some trees
        self.map.set_tile(10, 5, Tile::new(TerrainType::Tree));
        self.map.set_tile(11, 5, Tile::new(TerrainType::Tree));
        self.map.set_tile(10, 6, Tile::new(TerrainType::Tree));

        // Add some stones
        self.map.set_tile(15, 8, Tile::new(TerrainType::Stone));
        self.map.set_tile(16, 9, Tile::new(TerrainType::Stone));

        // Add some wood
        self.map.set_tile(8, 12, Tile::new(TerrainType::Wood));
        self.map.set_tile(9, 13, Tile::new(TerrainType::Wood));

        // Add weeds
        for i in 0..5 {
            self.map.set_tile(3 + i, 14, Tile::new(TerrainType::Weeds));
        }

        // Add farmhouse
        self.map.set_tile(2, 2, Tile::new(TerrainType::Farmhouse));
        self.map.set_tile(3, 2, Tile::new(TerrainType::Farmhouse));
        self.map.set_tile(2, 3, Tile::new(TerrainType::Farmhouse));
        self.map.set_tile(3, 3, Tile::new(TerrainType::Farmhouse));
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> Result<(), Box<dyn std::error::Error>> {
        let new_x = self.player.x as i32 + dx;
        let new_y = self.player.y as i32 + dy;

        if new_x >= 0 && new_x < self.map.width() as i32 &&
           new_y >= 0 && new_y < self.map.height() as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if let Some(tile) = self.map.get_tile(new_x, new_y) {
                if tile.is_walkable() {
                    self.player.x = new_x;
                    self.player.y = new_y;
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
