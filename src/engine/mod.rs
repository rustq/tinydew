use crate::input::InputManager;
use crate::rendering::Renderer;
use crate::time::TimeManager;
use crate::world::World;
use crate::tools::{Tool, ToolSlot};
use crate::inventory::{Inventory, ItemType};
use crate::farming::CropType;
use crossterm::event::{Event, KeyCode};
use std::error::Error;
use std::time::Duration;

pub struct Engine {
    input_manager: InputManager,
    renderer: Renderer,
    time_manager: TimeManager,
    world: World,
    running: bool,
    tool_slots: [ToolSlot; 9],
    current_tool_slot: usize,
    inventory: Inventory,
    money: u32,
}

impl Engine {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let input_manager = InputManager::new()?;
        let renderer = Renderer::new()?;
        let time_manager = TimeManager::new();
        let world = World::new()?;

        // Initialize tool slots
        let mut tool_slots = [ToolSlot::new(); 9];
        tool_slots[0].set_tool(Tool::Hoe);
        tool_slots[1].set_tool(Tool::WateringCan);
        tool_slots[2].set_tool(Tool::Axe);
        tool_slots[3].set_tool(Tool::Pickaxe);
        tool_slots[4].set_tool(Tool::FishingRod);

        // Initialize inventory with some seeds
        let mut inventory = Inventory::new();
        inventory.add(ItemType::Seed(CropType::Strawberry), 10);
        inventory.add(ItemType::Seed(CropType::Corn), 10);
        inventory.add(ItemType::Seed(CropType::Carrot), 10);

        Ok(Self {
            input_manager,
            renderer,
            time_manager,
            world,
            running: true,
            tool_slots,
            current_tool_slot: 0,
            inventory,
            money: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.renderer.init()?;

        while self.running {
            // Process input
            if let Some(event) = self.input_manager.poll()? {
                self.handle_event(event)?;
            }

            // Update time
            self.time_manager.tick();

            // Update crops daily
            if self.time_manager.game_time.minute == 0 && self.time_manager.game_time.hour == 6 {
                self.update_crops();
            }

            // Render
            self.renderer.render(&self.world, &self.time_manager, &self.inventory, self.money)?;

            // Sleep to maintain ~2.5s per game minute
            std::thread::sleep(Duration::from_millis(2500));
        }

        self.renderer.cleanup()?;
        Ok(())
    }

    fn update_crops(&mut self) {
        let map = self.world.map_mut();
        let current_day = self.time_manager.game_time.day;
        let weather = self.time_manager.game_time.weather;
        let is_rainy = weather.is_rainy();
        let is_snowy = weather.is_snowy();

        for y in 0..map.height() {
            for x in 0..map.width() {
                if let Some(tile) = map.get_tile_mut(x, y) {
                    if let Some(ref mut crop) = tile.crop {
                        // Reset watered flag for new day
                        if let Some(last_day) = tile.last_watered_day {
                            if last_day != current_day {
                                tile.watered = false;
                            }
                        }

                        // Auto-water on rainy days
                        if is_rainy && !tile.watered {
                            tile.watered = true;
                            tile.last_watered_day = Some(current_day);
                            crop.watered_today = true;
                        }

                        // Skip crop growth on snowy days
                        if is_snowy {
                            continue;
                        }

                        // Advance crop growth
                        crop.advance_day();
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn Error>> {
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Esc => self.running = false,
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                        self.world.move_player(0, -1)?;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                        self.world.move_player(0, 1)?;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
                        self.world.move_player(-1, 0)?;
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
                        self.world.move_player(1, 0)?;
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.use_tool()?;
                    }
                    KeyCode::Char('1') => self.current_tool_slot = 0,
                    KeyCode::Char('2') => self.current_tool_slot = 1,
                    KeyCode::Char('3') => self.current_tool_slot = 2,
                    KeyCode::Char('4') => self.current_tool_slot = 3,
                    KeyCode::Char('5') => self.current_tool_slot = 4,
                    KeyCode::Char('6') => self.current_tool_slot = 5,
                    KeyCode::Char('7') => self.current_tool_slot = 6,
                    KeyCode::Char('8') => self.current_tool_slot = 7,
                    KeyCode::Char('9') => self.current_tool_slot = 8,
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn use_tool(&mut self) -> Result<(), Box<dyn Error>> {
        let current_tool = self.tool_slots[self.current_tool_slot].get_tool();

        if let Some(tool) = current_tool {
            match tool {
                Tool::Hoe => self.use_hoe()?,
                Tool::WateringCan => self.use_watering_can()?,
                Tool::Axe => self.use_axe()?,
                Tool::Pickaxe => self.use_pickaxe()?,
                Tool::FishingRod => self.use_fishing_rod()?,
            }
        }

        Ok(())
    }

    fn use_hoe(&mut self) -> Result<(), Box<dyn Error>> {
        let player_pos = {
            let player = self.world.player();
            (player.x, player.y)
        };
        let map = self.world.map_mut();

        // Try to till adjacent tiles
        let adjacent_positions = [
            (player_pos.0.saturating_add(1), player_pos.1),
            (player_pos.0.saturating_sub(1), player_pos.1),
            (player_pos.0, player_pos.1.saturating_add(1)),
            (player_pos.0, player_pos.1.saturating_sub(1)),
        ];

        for (x, y) in adjacent_positions.iter() {
            if let Some(tile) = map.get_tile_mut(*x, *y) {
                if tile.is_walkable() {
                    // Convert grass to soil
                    tile.terrain = crate::world::TerrainType::Soil;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn use_watering_can(&mut self) -> Result<(), Box<dyn Error>> {
        let player_pos = {
            let player = self.world.player();
            (player.x, player.y)
        };
        let map = self.world.map_mut();
        let current_day = self.time_manager.game_time.day;

        // Try to water adjacent tiles
        let adjacent_positions = [
            (player_pos.0.saturating_add(1), player_pos.1),
            (player_pos.0.saturating_sub(1), player_pos.1),
            (player_pos.0, player_pos.1.saturating_add(1)),
            (player_pos.0, player_pos.1.saturating_sub(1)),
        ];

        for (x, y) in adjacent_positions.iter() {
            if let Some(tile) = map.get_tile_mut(*x, *y) {
                if let Some(ref mut crop) = tile.crop {
                    // Check if already watered today
                    if let Some(last_day) = tile.last_watered_day {
                        if last_day == current_day {
                            continue; // Already watered today
                        }
                    }

                    // Water the crop
                    tile.watered = true;
                    tile.last_watered_day = Some(current_day);
                    crop.watered_today = true;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn use_axe(&mut self) -> Result<(), Box<dyn Error>> {
        let player_pos = {
            let player = self.world.player();
            (player.x, player.y)
        };
        let map = self.world.map_mut();

        // Try to clear wood
        let adjacent_positions = [
            (player_pos.0.saturating_add(1), player_pos.1),
            (player_pos.0.saturating_sub(1), player_pos.1),
            (player_pos.0, player_pos.1.saturating_add(1)),
            (player_pos.0, player_pos.1.saturating_sub(1)),
        ];

        for (x, y) in adjacent_positions.iter() {
            if let Some(tile) = map.get_tile_mut(*x, *y) {
                if tile.terrain == crate::world::TerrainType::Wood {
                    // Clear wood and add to inventory
                    tile.terrain = crate::world::TerrainType::Grass;
                    self.inventory.add(ItemType::Wood, 1);
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn use_pickaxe(&mut self) -> Result<(), Box<dyn Error>> {
        let player_pos = {
            let player = self.world.player();
            (player.x, player.y)
        };
        let map = self.world.map_mut();

        // Try to clear stone
        let adjacent_positions = [
            (player_pos.0.saturating_add(1), player_pos.1),
            (player_pos.0.saturating_sub(1), player_pos.1),
            (player_pos.0, player_pos.1.saturating_add(1)),
            (player_pos.0, player_pos.1.saturating_sub(1)),
        ];

        for (x, y) in adjacent_positions.iter() {
            if let Some(tile) = map.get_tile_mut(*x, *y) {
                if tile.terrain == crate::world::TerrainType::Stone {
                    // Clear stone and add to inventory
                    tile.terrain = crate::world::TerrainType::Grass;
                    self.inventory.add(ItemType::Stone, 1);
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn use_fishing_rod(&mut self) -> Result<(), Box<dyn Error>> {
        let player_pos = {
            let player = self.world.player();
            (player.x, player.y)
        };
        let map = self.world.map();

        // Check if near water
        let adjacent_positions = [
            (player_pos.0.saturating_add(1), player_pos.1),
            (player_pos.0.saturating_sub(1), player_pos.1),
            (player_pos.0, player_pos.1.saturating_add(1)),
            (player_pos.0, player_pos.1.saturating_sub(1)),
        ];

        for (x, y) in adjacent_positions.iter() {
            if let Some(tile) = map.get_tile(*x, *y) {
                if tile.terrain == crate::world::TerrainType::Water {
                    // Fishing is possible (simplified for MVP)
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}
