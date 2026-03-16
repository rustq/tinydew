use crate::world::{
    Direction, EAST_PATH_HEIGHT, EAST_PATH_WIDTH, FARM_HEIGHT, FARM_WIDTH, Map, TileType,
    create_east_path_map, create_farm_map,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Farm,
    EastPath,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub location: Location,
    pub farm_map: Map,
    pub east_path_map: Map,
    pub player_x: usize,
    pub player_y: usize,
    pub direction: Direction,
    pub message: String,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub season: String,
    pub weather: String,
}

impl GameState {
    pub fn new() -> Self {
        let mut farm_map = create_farm_map();
        let (player_x, player_y) = find_player_start(&farm_map);

        let mut east_path_map = create_east_path_map();

        Self {
            location: Location::Farm,
            farm_map,
            east_path_map,
            player_x,
            player_y,
            direction: Direction::Down,
            message: String::from("Welcome to Shelldew!"),
            day: 1,
            hour: 6,
            minute: 0,
            season: String::from("Spring"),
            weather: String::from("Sunny"),
        }
    }

    pub fn get_current_map_ref(&self) -> &Map {
        match self.location {
            Location::Farm => &self.farm_map,
            Location::EastPath => &self.east_path_map,
        }
    }

    pub fn get_current_map(&mut self) -> &mut Map {
        match self.location {
            Location::Farm => &mut self.farm_map,
            Location::EastPath => &mut self.east_path_map,
        }
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        match self.location {
            Location::Farm => (FARM_WIDTH, FARM_HEIGHT),
            Location::EastPath => (EAST_PATH_WIDTH, EAST_PATH_HEIGHT),
        }
    }

    pub fn tile_in_front(&self) -> Option<(usize, usize)> {
        let (dx, dy) = self.direction.delta();
        let (width, height) = self.get_map_size();

        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    pub fn can_move_to(&self, x: usize, y: usize) -> bool {
        let (width, height) = self.get_map_size();
        if x >= width || y >= height {
            return false;
        }

        let map = self.get_current_map_ref();
        map[y][x].is_walkable()
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<TileType> {
        let (width, height) = self.get_map_size();
        if x >= width || y >= height {
            return None;
        }
        let map = self.get_current_map_ref();
        Some(map[y][x])
    }

    pub fn move_player(&mut self, direction: Direction) -> bool {
        self.direction = direction;

        let (dx, dy) = direction.delta();
        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            return false;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if self.can_move_to(new_x, new_y) {
            let target_tile = self.get_tile_at(new_x, new_y);

            if let Some(tile) = target_tile {
                if tile.is_transition() {
                    self.handle_transition(&tile);
                } else {
                    self.player_x = new_x;
                    self.player_y = new_y;
                    self.advance_time();
                }
            }
            true
        } else {
            self.message = String::from("Cannot move there!");
            false
        }
    }

    fn handle_transition(&mut self, tile: &TileType) {
        match (self.location, tile) {
            (Location::Farm, TileType::PathEast) => {
                self.location = Location::EastPath;
                self.player_x = 1;
                self.player_y = 2;
                self.direction = Direction::Right;
                self.message = String::from("Welcome to East Path!");
            }
            (Location::EastPath, TileType::PathFarm) => {
                self.location = Location::Farm;
                self.player_x = 7;
                self.player_y = 5;
                self.direction = Direction::Left;
                self.message = String::from("Back at the farm!");
            }
            _ => {}
        }
        self.advance_time();
    }

    pub fn advance_time(&mut self) {
        self.minute += 5;
        if self.minute >= 60 {
            self.minute = 0;
            self.hour += 1;
        }
        if self.hour >= 24 {
            self.hour = 0;
            self.day += 1;
        }
    }

    pub fn is_night(&self) -> bool {
        self.hour >= 20 || self.hour < 6
    }

    pub fn get_weather_icon(&self) -> &'static str {
        if self.is_night() {
            "🌙"
        } else {
            match self.weather.as_str() {
                "Sunny" => "☀️",
                "Rain" => "🌧",
                "Cloudy" => "☁️",
                _ => "☀️",
            }
        }
    }

    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn clear_action(&mut self) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot clear here!");
            return;
        }

        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Grass) = tile {
                self.farm_map[y][x] = TileType::Soil;
                self.message = String::from("Clear Done!");
                self.advance_time();
            } else {
                self.message = String::from("Nothing to clear!");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn plant_action(&mut self) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot plant here!");
            return;
        }

        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Soil) = tile {
                self.farm_map[y][x] = TileType::Crop(crate::world::CropType::Carrot, 0);
                self.message = String::from("Plant Done!");
                self.advance_time();
            } else {
                self.message = String::from("Cannot plant there!");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn water_action(&mut self) {
        if self.location != Location::Farm {
            self.message = String::from("Cannot water here!");
            return;
        }

        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Crop(_, _)) = tile {
                self.message = String::from("Water Done!");
                self.advance_time();
            } else {
                self.message = String::from("Nothing to water!");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn harvest_action(&mut self) {
        if let Some((x, y)) = self.tile_in_front() {
            let tile = self.get_tile_at(x, y);
            if let Some(TileType::Crop(_, stage)) = tile {
                if stage > 0 {
                    if self.location == Location::Farm {
                        self.farm_map[y][x] = TileType::Soil;
                    } else {
                        self.east_path_map[y][x] = TileType::Grass;
                    }
                    self.message = String::from("Harvest Done!");
                    self.advance_time();
                } else {
                    self.message = String::from("Not ready yet!");
                }
            } else {
                self.message = String::from("Nothing to harvest!");
            }
        } else {
            self.message = String::from("Nothing in front!");
        }
    }

    pub fn trade_action(&mut self) {
        self.message = String::from("Trade menu coming soon!");
        self.advance_time();
    }
}

fn find_player_start(map: &Map) -> (usize, usize) {
    (3, 3)
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
