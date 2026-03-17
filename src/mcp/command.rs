use crate::mcp::errors::{ErrorCode, McpError};
use crate::savegame;
use crate::state::{GameState, Location};
use crate::world::{CropState, CropType, Direction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub message: String,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_delta: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_text: Option<String>,
}

impl CommandResult {
    pub fn new(message: String) -> Self {
        Self {
            message,
            events: vec![],
            state_delta: None,
            snapshot_text: None,
        }
    }

    pub fn with_events(mut self, events: Vec<String>) -> Self {
        self.events = events;
        self
    }

    pub fn with_state_delta(mut self, delta: serde_json::Value) -> Self {
        self.state_delta = Some(delta);
        self
    }

    pub fn with_snapshot(mut self, snapshot: String) -> Self {
        self.snapshot_text = Some(snapshot);
        self
    }
}

#[derive(Debug, Clone)]
pub enum ParsedCommand {
    Move(Direction),
    Clear(Option<Direction>),
    Plant(CropType, Option<Direction>),
    Water(Option<Direction>),
    Harvest(Option<Direction>),
    Buy(CropType, u32),
    Sell(CropType, u32),
    Sleep,
    Print,
    Save,
    Load,
}

pub fn parse_command(input: &str) -> Result<ParsedCommand, McpError> {
    let input = input.trim().to_lowercase();
    let parts: Vec<&str> = input.splitn(2, ':').collect();
    let cmd = parts[0];
    let arg = parts.get(1).copied();

    match cmd {
        "move" => {
            let direction = arg.ok_or_else(|| {
                McpError::validation_error(
                    "move requires direction",
                    vec!["up", "down", "left", "right"],
                )
            })?;
            let dir = match direction {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "left" => Direction::Left,
                "right" => Direction::Right,
                _ => {
                    return Err(McpError::validation_error(
                        format!("invalid direction '{}'", direction),
                        vec!["up", "down", "left", "right"],
                    ));
                }
            };
            Ok(ParsedCommand::Move(dir))
        }
        "clear" => {
            let direction = arg.map(parse_direction).transpose()?;
            Ok(ParsedCommand::Clear(direction))
        }
        "plant" => {
            let (crop_str, dir_str) = if let Some(arg) = arg {
                let parts: Vec<&str> = arg.rsplitn(2, ':').collect();
                if parts.len() == 2 {
                    let dir = parse_direction(parts[0]);
                    if dir.is_ok() {
                        (parts[1], Some(parts[0]))
                    } else {
                        (arg, None)
                    }
                } else {
                    (arg, None)
                }
            } else {
                ("", None)
            };
            let crop_str = if crop_str.is_empty() {
                return Err(McpError::validation_error(
                    "plant requires crop type",
                    CropType::all().iter().map(|c| c.seed_name()).collect(),
                ));
            } else {
                crop_str
            };
            let crop = parse_crop(crop_str)?;
            let direction = dir_str.map(parse_direction).transpose()?;
            Ok(ParsedCommand::Plant(crop, direction))
        }
        "water" => {
            let direction = arg.map(parse_direction).transpose()?;
            Ok(ParsedCommand::Water(direction))
        }
        "harvest" => {
            let direction = arg.map(parse_direction).transpose()?;
            Ok(ParsedCommand::Harvest(direction))
        }
        "buy" => {
            let (item_str, qty) = parse_item_with_qty(arg.unwrap_or(""))?;
            let crop = parse_crop(item_str)?;
            Ok(ParsedCommand::Buy(crop, qty))
        }
        "sell" => {
            let (item_str, qty) = parse_item_with_qty(arg.unwrap_or(""))?;
            let crop = parse_crop(item_str)?;
            Ok(ParsedCommand::Sell(crop, qty))
        }
        "sleep" => Ok(ParsedCommand::Sleep),
        "print" => Ok(ParsedCommand::Print),
        "save" => Ok(ParsedCommand::Save),
        "load" => Ok(ParsedCommand::Load),
        _ => Err(McpError::invalid_command(format!(
            "unknown command '{}'. Valid commands: move:up|down|left|right, clear[:<dir>], plant:<crop>[:<dir>], water[:<dir>], harvest[:<dir>], buy:<item>[:<qty>], sell:<item>[:<qty>], sleep, print, save, load",
            cmd
        ))),
    }
}

fn parse_crop(s: &str) -> Result<CropType, McpError> {
    match s {
        "carrot" => Ok(CropType::Carrot),
        "strawberry" => Ok(CropType::Strawberry),
        "cauliflower" => Ok(CropType::Cauliflower),
        "rhubarb" => Ok(CropType::Rhubarb),
        _ => Err(McpError::validation_error(
            format!("invalid crop '{}'", s),
            vec!["carrot", "strawberry", "cauliflower", "rhubarb"],
        )),
    }
}

fn parse_direction(s: &str) -> Result<Direction, McpError> {
    match s {
        "up" => Ok(Direction::Up),
        "down" => Ok(Direction::Down),
        "left" => Ok(Direction::Left),
        "right" => Ok(Direction::Right),
        _ => Err(McpError::validation_error(
            format!("invalid direction '{}'", s),
            vec!["up", "down", "left", "right"],
        )),
    }
}

fn parse_item_with_qty(s: &str) -> Result<(&str, u32), McpError> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    let item = parts[0];
    if item.is_empty() {
        return Err(McpError::validation_error(
            "item is required for buy/sell",
            vec!["carrot", "strawberry", "cauliflower", "rhubarb"],
        ));
    }
    let qty = if let Some(qty_str) = parts.get(1) {
        qty_str.parse::<u32>().map_err(|_| {
            McpError::validation_error(
                "quantity must be a positive integer",
                vec!["1", "2", "5", "10"],
            )
        })?
    } else {
        1
    };
    if qty == 0 {
        return Err(McpError::validation_error(
            "quantity must be a positive integer",
            vec!["1", "2", "5", "10"],
        ));
    }
    Ok((item, qty))
}

fn capture_state_snapshot(state: &GameState) -> serde_json::Value {
    serde_json::json!({
        "day": state.day,
        "time": state.format_time(),
        "location": format!("{:?}", state.location),
        "money": state.money,
        "inventory": {
            "seeds": state.inventory.seeds,
            "produce": state.inventory.produce,
            "forage": state.inventory.forage,
        },
        "player": {
            "x": state.player_x,
            "y": state.player_y,
        }
    })
}

fn advance_to_morning(state: &mut GameState) {
    state.day += 1;
    state.hour = 6;
    state.minute = 0;

    state.location = Location::Farm;
    state.player_x = 3;
    state.player_y = 3;

    state.start_new_day();

    state.home_state = crate::state::HomeState::None;
    state.current_income = crate::state::DailyIncome::default();
    state.message = String::from("Good morning! Ready for another day.");
}

fn generate_text_snapshot(state: &GameState) -> String {
    let mut lines = vec![
        format!("=== Shelldew Day {} {} ===", state.day, state.format_time()),
        format!("Location: {:?}", state.location),
        format!("Money: ${}", state.money),
        String::new(),
        "--- Player ---".to_string(),
        format!("Position: ({}, {})", state.player_x, state.player_y),
        String::new(),
        "--- Inventory ---".to_string(),
    ];

    if state.inventory.seeds.is_empty()
        && state.inventory.produce.is_empty()
        && state.inventory.forage.is_empty()
    {
        lines.push("(empty)".to_string());
    } else {
        for (crop, count) in &state.inventory.seeds {
            if *count > 0 {
                lines.push(format!("  Seeds: {} x{}", crop.seed_name(), count));
            }
        }
        for (crop, count) in &state.inventory.produce {
            if *count > 0 {
                lines.push(format!(
                    "  Produce: {} {} x{}",
                    crop.produce_emoji(),
                    crop.seed_name(),
                    count
                ));
            }
        }
        for (forage, count) in &state.inventory.forage {
            if *count > 0 {
                lines.push(format!("  Forage: {} x{}", forage.emoji(), count));
            }
        }
    }

    lines.push(String::new());
    lines.push("--- Farm Map ---".to_string());

    let map = state.get_current_map_ref();
    let (width, height) = state.get_map_size();
    for y in 0..height {
        let line: String = (0..width)
            .map(|x| {
                if x == state.player_x && y == state.player_y {
                    "🧑"
                } else {
                    map[y][x].emoji()
                }
            })
            .collect();
        lines.push(line);
    }

    if !state.message.is_empty() {
        lines.push(String::new());
        lines.push(format!("> {}", state.message));
    }

    lines.join("\n")
}

pub fn execute_command(state: &mut GameState, cmd: ParsedCommand) -> CommandResult {
    match cmd {
        ParsedCommand::Move(direction) => {
            let old_x = state.player_x;
            let old_y = state.player_y;
            let moved = state.move_player(direction);
            let new_x = state.player_x;
            let new_y = state.player_y;

            let event = if moved && (old_x != new_x || old_y != new_y) {
                format!("Moved {:?} to ({}, {})", direction, new_x, new_y)
            } else {
                state.message.clone()
            };

            CommandResult::new(state.message.clone())
                .with_events(vec![event])
                .with_state_delta(serde_json::json!({
                    "player": { "x": new_x, "y": new_y }
                }))
        }
        ParsedCommand::Clear(dir) => {
            match dir {
                Some(d) => state.clear_action_at(d),
                None => state.clear_action(),
            };
            CommandResult::new(state.message.clone()).with_events(vec!["Cleared tile".to_string()])
        }
        ParsedCommand::Plant(crop, dir) => {
            let original_message = state.message.clone();
            state.selected_seed = crop;
            match dir {
                Some(d) => state.plant_action_at(d),
                None => state.plant_action(),
            };

            let events = if state.message.contains("Done") || original_message != state.message {
                vec![format!("Planted {}", crop.seed_name())]
            } else {
                vec![]
            };

            CommandResult::new(state.message.clone())
                .with_events(events)
                .with_state_delta(serde_json::json!({
                    "selected_seed": crop.seed_name()
                }))
        }
        ParsedCommand::Water(dir) => {
            match dir {
                Some(d) => state.water_action_at(d),
                None => state.water_action(),
            };
            CommandResult::new(state.message.clone()).with_events(vec!["Watered crop".to_string()])
        }
        ParsedCommand::Harvest(dir) => {
            match dir {
                Some(d) => state.harvest_action_at(d),
                None => state.harvest_action(),
            };
            CommandResult::new(state.message.clone()).with_events(vec!["Harvested".to_string()])
        }
        ParsedCommand::Buy(crop, qty) => {
            let price = crop.seed_price() * qty;
            if state.money >= price {
                state.money -= price;
                state.inventory.add_seeds(crop, qty);
                state.message = format!("Bought {} x{} for ${}!", crop.seed_name(), qty, price);
            } else {
                state.message = format!("Not enough money! Need ${}, have ${}", price, state.money);
            }

            CommandResult::new(state.message.clone())
                .with_events(vec![format!("Bought {} seeds", crop.seed_name())])
                .with_state_delta(serde_json::json!({
                    "money": state.money,
                    "seeds": state.inventory.seeds
                }))
        }
        ParsedCommand::Sell(crop, qty) => {
            let mut sold_count = 0;
            for _ in 0..qty {
                if state.inventory.sell_produce(crop) {
                    sold_count += 1;
                } else {
                    break;
                }
            }

            if sold_count > 0 {
                let revenue = crop.produce_price() * sold_count;
                state.money += revenue;
                state.message = format!(
                    "Sold {} x{} for ${}!",
                    crop.produce_emoji(),
                    sold_count,
                    revenue
                );
            } else {
                state.message = format!("No {} produce to sell!", crop.seed_name());
            }

            CommandResult::new(state.message.clone())
                .with_events(vec![format!("Sold {} produce", sold_count)])
                .with_state_delta(serde_json::json!({
                    "money": state.money,
                    "produce": state.inventory.produce
                }))
        }
        ParsedCommand::Sleep => {
            advance_to_morning(state);

            CommandResult::new(state.message.clone())
                .with_events(vec!["Slept".to_string()])
                .with_state_delta(capture_state_snapshot(state))
        }
        ParsedCommand::Print => {
            let snapshot = generate_text_snapshot(state);
            CommandResult::new("Game state snapshot".to_string())
                .with_events(vec![])
                .with_snapshot(snapshot)
        }
        ParsedCommand::Save => match savegame::save_game(state) {
            Ok(path) => {
                let msg = format!("Game saved to {:?}", path);
                CommandResult::new(msg)
                    .with_events(vec!["Saved".to_string()])
                    .with_state_delta(serde_json::json!({
                        "saved": true,
                        "path": path.to_string_lossy()
                    }))
            }
            Err(e) => CommandResult::new(format!("Save failed: {}", e)).with_events(vec![]),
        },
        ParsedCommand::Load => match savegame::load_game() {
            Ok(loaded_state) => {
                *state = loaded_state;
                let msg = "Game loaded successfully!".to_string();
                CommandResult::new(msg)
                    .with_events(vec!["Loaded".to_string()])
                    .with_state_delta(capture_state_snapshot(state))
            }
            Err(e) => CommandResult::new(format!("Load failed: {}", e)).with_events(vec![]),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move_up() {
        let result = parse_command("move:up");
        assert!(matches!(result, Ok(ParsedCommand::Move(Direction::Up))));
    }

    #[test]
    fn test_parse_move_down() {
        let result = parse_command("move:down");
        assert!(matches!(result, Ok(ParsedCommand::Move(Direction::Down))));
    }

    #[test]
    fn test_parse_move_left() {
        let result = parse_command("move:left");
        assert!(matches!(result, Ok(ParsedCommand::Move(Direction::Left))));
    }

    #[test]
    fn test_parse_move_right() {
        let result = parse_command("move:right");
        assert!(matches!(result, Ok(ParsedCommand::Move(Direction::Right))));
    }

    #[test]
    fn test_parse_clear() {
        let result = parse_command("clear");
        assert!(matches!(result, Ok(ParsedCommand::Clear(None))));
    }

    #[test]
    fn test_parse_clear_with_direction() {
        let result = parse_command("clear:up");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Clear(Some(Direction::Up)))
        ));

        let result = parse_command("clear:down");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Clear(Some(Direction::Down)))
        ));

        let result = parse_command("clear:left");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Clear(Some(Direction::Left)))
        ));

        let result = parse_command("clear:right");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Clear(Some(Direction::Right)))
        ));
    }

    #[test]
    fn test_parse_plant() {
        let result = parse_command("plant:carrot");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Carrot, None))
        ));
    }

    #[test]
    fn test_parse_plant_with_direction() {
        let result = parse_command("plant:carrot:up");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Carrot, Some(Direction::Up)))
        ));

        let result = parse_command("plant:strawberry:down");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(
                CropType::Strawberry,
                Some(Direction::Down)
            ))
        ));

        let result = parse_command("plant:cauliflower:left");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(
                CropType::Cauliflower,
                Some(Direction::Left)
            ))
        ));

        let result = parse_command("plant:rhubarb:right");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(
                CropType::Rhubarb,
                Some(Direction::Right)
            ))
        ));
    }

    #[test]
    fn test_parse_water() {
        let result = parse_command("water");
        assert!(matches!(result, Ok(ParsedCommand::Water(None))));
    }

    #[test]
    fn test_parse_water_with_direction() {
        let result = parse_command("water:up");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Water(Some(Direction::Up)))
        ));

        let result = parse_command("water:down");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Water(Some(Direction::Down)))
        ));

        let result = parse_command("water:left");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Water(Some(Direction::Left)))
        ));

        let result = parse_command("water:right");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Water(Some(Direction::Right)))
        ));
    }

    #[test]
    fn test_parse_harvest() {
        let result = parse_command("harvest");
        assert!(matches!(result, Ok(ParsedCommand::Harvest(None))));
    }

    #[test]
    fn test_parse_harvest_with_direction() {
        let result = parse_command("harvest:up");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Harvest(Some(Direction::Up)))
        ));

        let result = parse_command("harvest:down");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Harvest(Some(Direction::Down)))
        ));

        let result = parse_command("harvest:left");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Harvest(Some(Direction::Left)))
        ));

        let result = parse_command("harvest:right");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Harvest(Some(Direction::Right)))
        ));
    }

    #[test]
    fn test_parse_invalid_direction() {
        let result = parse_command("clear:north");
        assert!(matches!(
            result,
            Err(McpError {
                code: ErrorCode::ValidationError,
                ..
            })
        ));

        let result = parse_command("water:south");
        assert!(matches!(
            result,
            Err(McpError {
                code: ErrorCode::ValidationError,
                ..
            })
        ));

        let result = parse_command("plant:carrot:east");
        assert!(matches!(
            result,
            Err(McpError {
                code: ErrorCode::ValidationError,
                ..
            })
        ));

        let result = parse_command("move:north");
        assert!(matches!(
            result,
            Err(McpError {
                code: ErrorCode::ValidationError,
                ..
            })
        ));
    }

    #[test]
    fn test_parse_buy() {
        let result = parse_command("buy:carrot");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Buy(CropType::Carrot, 1))
        ));
    }

    #[test]
    fn test_parse_buy_with_qty() {
        let result = parse_command("buy:carrot:5");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Buy(CropType::Carrot, 5))
        ));
    }

    #[test]
    fn test_parse_sell() {
        let result = parse_command("sell:carrot");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Sell(CropType::Carrot, 1))
        ));
    }

    #[test]
    fn test_parse_sell_with_qty() {
        let result = parse_command("sell:carrot:3");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Sell(CropType::Carrot, 3))
        ));
    }

    #[test]
    fn test_parse_sleep() {
        let result = parse_command("sleep");
        assert!(matches!(result, Ok(ParsedCommand::Sleep)));
    }

    #[test]
    fn test_parse_print() {
        let result = parse_command("print");
        assert!(matches!(result, Ok(ParsedCommand::Print)));
    }

    #[test]
    fn test_parse_save() {
        let result = parse_command("save");
        assert!(matches!(result, Ok(ParsedCommand::Save)));
    }

    #[test]
    fn test_parse_load() {
        let result = parse_command("load");
        assert!(matches!(result, Ok(ParsedCommand::Load)));
    }

    #[test]
    fn test_parse_invalid_command() {
        let result = parse_command("fly:away");
        assert!(matches!(
            result,
            Err(McpError {
                code: ErrorCode::InvalidCommand,
                ..
            })
        ));
    }

    #[test]
    fn test_parse_invalid_crop() {
        let result = parse_command("plant:tomato");
        assert!(matches!(
            result,
            Err(McpError {
                code: ErrorCode::ValidationError,
                ..
            })
        ));
    }

    #[test]
    fn test_execute_move() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Move(Direction::Down));
        assert!(!result.message.is_empty());
        assert!(!result.events.is_empty());
    }

    #[test]
    fn test_execute_print() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Print);
        assert!(result.snapshot_text.is_some());
        let snapshot = result.snapshot_text.unwrap();
        assert!(snapshot.contains("Day"));
    }

    #[test]
    fn test_print_snapshot_contains_player_marker() {
        let mut state = GameState::new();
        state.player_x = 2;
        state.player_y = 3;
        let result = execute_command(&mut state, ParsedCommand::Print);
        let snapshot = result.snapshot_text.unwrap();

        let map_start = snapshot
            .lines()
            .position(|l| l.starts_with("--- Farm Map ---"));
        assert!(map_start.is_some(), "Map section should exist");
        let map_start = map_start.unwrap();

        let map_rows: Vec<&str> = snapshot.lines().skip(map_start + 1).take(8).collect();

        assert_eq!(map_rows.len(), 8, "Should have 8 map rows");

        let row_3 = map_rows.get(3);
        assert!(row_3.is_some(), "Row 3 should exist");

        assert!(
            row_3.unwrap().contains("🧑"),
            "Player marker 🧑 should appear in row 3"
        );

        let char_count = row_3.unwrap().chars().count();
        assert_eq!(char_count, 8, "Row should have 8 characters");

        let player_char_idx = row_3.unwrap().chars().position(|c| c == '🧑').unwrap();
        assert_eq!(
            player_char_idx, 2,
            "Player marker should be at character index 2 (x=2) in row 3, got {}",
            player_char_idx
        );
    }

    #[test]
    fn test_parse_empty_command() {
        let result = parse_command("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::InvalidCommand);
    }

    #[test]
    fn test_parse_whitespace_command() {
        let result = parse_command("   ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::InvalidCommand);
    }

    #[test]
    fn test_parse_move_missing_direction() {
        let result = parse_command("move");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ValidationError);
    }

    #[test]
    fn test_parse_plant_missing_crop() {
        let result = parse_command("plant");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ValidationError);
    }

    #[test]
    fn test_parse_case_insensitive() {
        let result = parse_command("MOVE:UP");
        assert!(matches!(result, Ok(ParsedCommand::Move(Direction::Up))));

        let result = parse_command("Move:Down");
        assert!(matches!(result, Ok(ParsedCommand::Move(Direction::Down))));

        let result = parse_command("plant:CARROT");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Carrot, None))
        ));
    }

    #[test]
    fn test_parse_buy_missing_item() {
        let result = parse_command("buy:");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_sell_missing_item() {
        let result = parse_command("sell:");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_buy_zero_quantity() {
        let result = parse_command("buy:carrot:0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::ValidationError);
    }

    #[test]
    fn test_parse_invalid_quantity() {
        let result = parse_command("buy:carrot:abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_all_crop_types() {
        let result = parse_command("plant:carrot");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Carrot, None))
        ));

        let result = parse_command("plant:strawberry");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Strawberry, None))
        ));

        let result = parse_command("plant:cauliflower");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Cauliflower, None))
        ));

        let result = parse_command("plant:rhubarb");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Rhubarb, None))
        ));
    }

    #[test]
    fn test_execute_plant() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Plant(CropType::Carrot, None));
        assert!(!result.message.is_empty());
        assert!(result.state_delta.is_some());
    }

    #[test]
    fn test_execute_water() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Water(None));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_harvest() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Harvest(None));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_clear() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Clear(None));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_directional_clear() {
        use crate::world::TileType;

        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 3;
        state.player_y = 3;
        state.farm_map[3][4] = TileType::Grass;

        let result = execute_command(&mut state, ParsedCommand::Clear(Some(Direction::Right)));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_directional_water() {
        use crate::world::TileType;

        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 3;
        state.player_y = 3;
        state.farm_map[3][4] = TileType::Crop(
            CropType::Carrot,
            CropState {
                days_grown: 1,
                watered_today: false,
            },
        );

        let result = execute_command(&mut state, ParsedCommand::Water(Some(Direction::Right)));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_directional_harvest() {
        use crate::world::TileType;

        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 3;
        state.player_y = 3;
        state.farm_map[3][4] = TileType::Crop(
            CropType::Carrot,
            CropState {
                days_grown: 4,
                watered_today: true,
            },
        );

        let result = execute_command(&mut state, ParsedCommand::Harvest(Some(Direction::Right)));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_directional_plant() {
        use crate::world::TileType;

        let mut state = GameState::new();
        state.location = Location::Farm;
        state.player_x = 3;
        state.player_y = 3;
        state.farm_map[3][4] = TileType::Soil;
        state.inventory.seeds.insert(CropType::Carrot, 5);

        let result = execute_command(
            &mut state,
            ParsedCommand::Plant(CropType::Carrot, Some(Direction::Right)),
        );
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_sleep() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Sleep);
        assert!(!result.message.is_empty());
        assert!(result.state_delta.is_some());
    }

    #[test]
    fn test_sleep_advances_day_and_time() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 22;
        state.minute = 30;

        let original_day = state.day;
        let original_time = state.format_time();

        execute_command(&mut state, ParsedCommand::Sleep);

        assert!(
            state.day > original_day || state.format_time() != original_time,
            "Sleep should advance day or time. Before: Day {} {}, After: Day {} {}",
            original_day,
            original_time,
            state.day,
            state.format_time()
        );
    }

    #[test]
    fn test_sleep_transitions_to_next_morning() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 20;
        state.minute = 0;

        execute_command(&mut state, ParsedCommand::Sleep);

        assert_eq!(
            state.day,
            2,
            "After sleep, day should be exactly 2. Got: Day {} {}",
            state.day,
            state.format_time()
        );
        assert!(
            state.hour >= 6 && state.hour < 12,
            "After sleep, time should be morning (6-12). Got: Day {} {}",
            state.day,
            state.format_time()
        );
    }

    #[test]
    fn test_sleep_at_daytime_increments_day() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 14;
        state.minute = 30;

        execute_command(&mut state, ParsedCommand::Sleep);

        assert_eq!(
            state.day,
            2,
            "Sleep at daytime (14:30) should increment day. Got: Day {} {}",
            state.day,
            state.format_time()
        );
        assert_eq!(
            state.hour, 6,
            "After sleep, hour should be 6. Got: {}",
            state.hour
        );
    }

    #[test]
    fn test_sleep_at_morning_increments_day() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 6;
        state.minute = 0;

        execute_command(&mut state, ParsedCommand::Sleep);

        assert_eq!(
            state.day,
            2,
            "Sleep at morning (6:00) should increment day. Got: Day {} {}",
            state.day,
            state.format_time()
        );
    }

    #[test]
    fn test_post_sleep_state_is_playable() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 20;
        state.minute = 0;

        execute_command(&mut state, ParsedCommand::Sleep);

        assert_eq!(
            state.home_state,
            crate::state::HomeState::None,
            "After sleep, home_state should be None (playable)"
        );
        assert!(
            state.message.contains("morning") || state.message.contains("day"),
            "Morning message expected. Got: {}",
            state.message
        );
    }

    #[test]
    fn test_sleep_from_farm_wakes_at_home_front() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 20;
        state.minute = 0;
        state.location = Location::Farm;
        state.player_x = 5;
        state.player_y = 4;

        execute_command(&mut state, ParsedCommand::Sleep);

        assert_eq!(
            state.location,
            Location::Farm,
            "After sleep from Farm, location should be Farm"
        );
        assert_eq!(
            state.player_x, 3,
            "After sleep, player_x should be 3 (home-front). Got: {}",
            state.player_x
        );
        assert_eq!(
            state.player_y, 3,
            "After sleep, player_y should be 3 (home-front). Got: {}",
            state.player_y
        );
    }

    #[test]
    fn test_sleep_from_east_path_wakes_at_home_front() {
        let mut state = GameState::new();
        state.day = 1;
        state.hour = 20;
        state.minute = 0;
        state.location = Location::EastPath;
        state.player_x = 5;
        state.player_y = 2;

        execute_command(&mut state, ParsedCommand::Sleep);

        assert_eq!(
            state.location,
            Location::Farm,
            "After sleep from EastPath, location should be Farm"
        );
        assert_eq!(
            state.player_x, 3,
            "After sleep, player_x should be 3 (home-front). Got: {}",
            state.player_x
        );
        assert_eq!(
            state.player_y, 3,
            "After sleep, player_y should be 3 (home-front). Got: {}",
            state.player_y
        );
    }

    #[test]
    fn test_execute_buy() {
        let mut state = GameState::new();
        state.money = 100;
        let result = execute_command(&mut state, ParsedCommand::Buy(CropType::Carrot, 5));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_sell() {
        let mut state = GameState::new();
        state.inventory.produce.insert(CropType::Carrot, 5);
        let result = execute_command(&mut state, ParsedCommand::Sell(CropType::Carrot, 3));
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_mcp_command_batch_water_sleep_cycle_without_buying() {
        use crate::mcp::handler::{handle_command, handle_end_session, handle_start_session};
        use crate::mcp::tools::{CommandInput, EndSessionInput, StartSessionInput};

        let start_input = StartSessionInput {
            seed: Some(42),
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        assert!(start_response.ok);
        let session_id = serde_json::from_value::<crate::mcp::tools::StartSessionOutput>(
            start_response.result.unwrap(),
        )
        .unwrap()
        .session_id;

        // Try to plant without buying seeds first - should fail because no soil
        let cmd_plant = CommandInput {
            session_id: session_id.clone(),
            command: "plant:carrot".to_string(),
        };
        let plant_resp = handle_command(cmd_plant);

        // Should fail because no soil (player starts on grass)
        let result = plant_resp.result.as_ref().unwrap();
        let message = result.get("message").unwrap().as_str().unwrap();
        assert!(
            message.contains("Cannot plant there!") || message.contains("No Carrot seeds"),
            "Got: {}",
            message
        );

        // Clean up
        let _ = handle_end_session(EndSessionInput { session_id });
    }

    #[test]
    fn test_command_batch_crop_growth() {
        use crate::mcp::handler::{handle_command_batch, handle_end_session, handle_start_session};
        use crate::mcp::tools::{
            CommandBatchInput, EndSessionInput, GetMapInput, StartSessionInput,
        };

        let start_input = StartSessionInput {
            seed: Some(42),
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        let session_id = serde_json::from_value::<crate::mcp::tools::StartSessionOutput>(
            start_response.result.unwrap(),
        )
        .unwrap()
        .session_id;

        // First prepare: move, clear, buy seeds, plant
        let batch_prepare = CommandBatchInput {
            session_id: session_id.clone(),
            commands: vec![
                "move:down".to_string(),
                "clear".to_string(),
                "buy:carrot:5".to_string(),
                "plant:carrot".to_string(),
            ],
            stop_on_error: true,
        };
        let _ = handle_command_batch(batch_prepare);

        // Then water + sleep for 4 days
        // Note: After planting, player at (3,4) can water (3,5) without moving.
        // After each sleep, player returns to home-front (3,3), so needs move:down to get to (3,4).
        let batch_grow = CommandBatchInput {
            session_id: session_id.clone(),
            commands: vec![
                "water".to_string(),
                "sleep".to_string(),
                "move:down".to_string(),
                "water".to_string(),
                "sleep".to_string(),
                "move:down".to_string(),
                "water".to_string(),
                "sleep".to_string(),
                "move:down".to_string(),
                "water".to_string(),
                "sleep".to_string(),
            ],
            stop_on_error: true,
        };
        let grow_resp = handle_command_batch(batch_grow);
        assert!(grow_resp.ok);

        // Check final crop state
        let map_input = GetMapInput {
            session_id: session_id.clone(),
            include_entities: Some(true),
        };
        let map_response = crate::mcp::handler::handle_get_map(map_input);
        let map_result = map_response.result.unwrap();
        let tiles = map_result.get("tiles").unwrap().as_array().unwrap();

        let mut crop_info = String::new();
        let mut found_mature = false;
        for row in tiles {
            for tile in row.as_array().unwrap() {
                let tile_str = tile.as_str().unwrap();
                crop_info = tile_str.to_string();
                if tile_str.contains("Crop") && tile_str.contains("mature") {
                    found_mature = true;
                }
            }
        }

        assert!(
            found_mature,
            "Expected mature crop after batch water+sleep cycles. Crop: {}",
            crop_info
        );

        let _ = handle_end_session(EndSessionInput { session_id });
    }

    #[test]
    fn test_water_and_sleep_crop_growth() {
        use crate::world::TileType;

        let mut state = GameState::new();
        state.hour = 6;
        state.minute = 0;
        state.player_x = 3;
        state.player_y = 3;
        state.direction = Direction::Down;

        state.farm_map[4][3] = TileType::Crop(CropType::Carrot, CropState::new());

        if let TileType::Crop(_, crop_state) = state.farm_map[4][3] {
            assert_eq!(crop_state.days_grown, 0);
            assert!(!crop_state.watered_today);
        }

        state.water_action();

        if let TileType::Crop(_, crop_state) = state.farm_map[4][3] {
            assert!(
                crop_state.watered_today,
                "Crop should be watered after water_action"
            );
        }

        execute_command(&mut state, ParsedCommand::Sleep);

        if let TileType::Crop(_crop, crop_state) = state.farm_map[4][3] {
            assert_eq!(
                crop_state.days_grown, 1,
                "Crop should have grown to 1 day after sleep, got {}",
                crop_state.days_grown
            );
            assert!(
                !crop_state.watered_today,
                "watered_today should be reset after sleep"
            );
        }

        for day in 1..4 {
            state.water_action();
            execute_command(&mut state, ParsedCommand::Sleep);

            if let TileType::Crop(_crop, crop_state) = state.farm_map[4][3] {
                assert_eq!(
                    crop_state.days_grown,
                    day + 1,
                    "After day {} sleep, days_grown should be {}, got {}",
                    day + 1,
                    day + 1,
                    crop_state.days_grown
                );
            }
        }

        if let TileType::Crop(crop, crop_state) = state.farm_map[4][3] {
            assert!(
                crop_state.is_mature(crop),
                "Carrot should be mature after 4 days of growth, days_grown={}",
                crop_state.days_grown
            );
        }
    }

    #[test]
    fn test_mcp_command_batch_water_sleep_cycle() {
        use crate::mcp::handler::{handle_command, handle_start_session};
        use crate::mcp::tools::{CommandInput, GetMapInput, GetStateInput, StartSessionInput};

        let start_input = StartSessionInput {
            seed: Some(42),
            mode: Some("test".to_string()),
        };
        let start_response = handle_start_session(start_input);
        assert!(start_response.ok);
        let session_id = serde_json::from_value::<crate::mcp::tools::StartSessionOutput>(
            start_response.result.unwrap(),
        )
        .unwrap()
        .session_id;

        let cmd_place = CommandInput {
            session_id: session_id.clone(),
            command: "move:down".to_string(),
        };
        let _ = handle_command(cmd_place);

        let cmd_clear = CommandInput {
            session_id: session_id.clone(),
            command: "clear".to_string(),
        };
        let _ = handle_command(cmd_clear);

        let cmd_buy = CommandInput {
            session_id: session_id.clone(),
            command: "buy:carrot:5".to_string(),
        };
        let _ = handle_command(cmd_buy);

        let cmd_plant = CommandInput {
            session_id: session_id.clone(),
            command: "plant:carrot".to_string(),
        };
        let _ = handle_command(cmd_plant);

        let cmd_water = CommandInput {
            session_id: session_id.clone(),
            command: "water".to_string(),
        };
        let _ = handle_command(cmd_water);

        let cmd_sleep = CommandInput {
            session_id: session_id.clone(),
            command: "sleep".to_string(),
        };
        let sleep_response = handle_command(cmd_sleep);
        assert!(sleep_response.ok);

        let state_input = GetStateInput {
            session_id: session_id.clone(),
        };
        let state_response = crate::mcp::handler::handle_get_state(state_input);
        let state = state_response.result.unwrap();
        assert_eq!(state.get("day").unwrap().as_u64().unwrap(), 2);

        for _ in 0..3 {
            // After each sleep, player returns to home-front (3,3)
            // Move down to get back to crop position (3,5)
            let cmd_move = CommandInput {
                session_id: session_id.clone(),
                command: "move:down".to_string(),
            };
            let _ = handle_command(cmd_move);

            let cmd_water = CommandInput {
                session_id: session_id.clone(),
                command: "water".to_string(),
            };
            let _ = handle_command(cmd_water);

            let cmd_sleep = CommandInput {
                session_id: session_id.clone(),
                command: "sleep".to_string(),
            };
            let _ = handle_command(cmd_sleep);
        }

        let map_input = GetMapInput {
            session_id: session_id.clone(),
            include_entities: Some(true),
        };
        let map_response = crate::mcp::handler::handle_get_map(map_input);
        let map_result = map_response.result.unwrap();
        let tiles = map_result.get("tiles").unwrap().as_array().unwrap();

        let mut found_crop = false;
        let mut crop_info = String::new();
        for row in tiles {
            for tile in row.as_array().unwrap() {
                let tile_str = tile.as_str().unwrap();
                if tile_str.contains("Crop") {
                    crop_info = tile_str.to_string();
                    if tile_str.contains("mature") {
                        found_crop = true;
                    }
                }
            }
        }

        assert!(
            found_crop,
            "Expected to find a mature crop after water+sleep cycles. Crop info: {}",
            crop_info
        );
    }
}
