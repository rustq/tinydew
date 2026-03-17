use crate::mcp::errors::{ErrorCode, McpError};
use crate::state::GameState;
use crate::world::{CropType, Direction};
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
    Clear,
    Plant(CropType),
    Water,
    Harvest,
    Buy(CropType, u32),
    Sell(CropType, u32),
    Sleep,
    Print,
}

pub fn parse_command(input: &str) -> Result<ParsedCommand, McpError> {
    let input = input.trim().to_lowercase();
    let parts: Vec<&str> = input.splitn(2, ':').collect();
    let cmd = parts[0];
    let arg = parts.get(1).map(|s| *s);

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
        "clear" => Ok(ParsedCommand::Clear),
        "plant" => {
            let crop_str = arg.ok_or_else(|| {
                McpError::validation_error(
                    "plant requires crop type",
                    CropType::all().iter().map(|c| c.seed_name()).collect(),
                )
            })?;
            let crop = parse_crop(crop_str)?;
            Ok(ParsedCommand::Plant(crop))
        }
        "water" => Ok(ParsedCommand::Water),
        "harvest" => Ok(ParsedCommand::Harvest),
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
        _ => Err(McpError::invalid_command(format!(
            "unknown command '{}'. Valid commands: move:up|down|left|right, clear, plant:<crop>, water, harvest, buy:<item>[:<qty>], sell:<item>[:<qty>], sleep, print",
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

fn generate_text_snapshot(state: &GameState) -> String {
    let mut lines = vec![
        format!("=== Shelldew Day {} {} ===", state.day, state.format_time()),
        format!("Location: {:?}", state.location),
        format!("Money: ${}", state.money),
        String::new(),
        "--- Player ---".to_string(),
        format!("Position: ({}, {})", state.player_x, state.player_y),
        format!("Direction: {:?}", state.direction),
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
    for row in map {
        let line: String = row.iter().map(|t| t.emoji()).collect();
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
        ParsedCommand::Clear => {
            state.clear_action();
            CommandResult::new(state.message.clone()).with_events(vec!["Cleared tile".to_string()])
        }
        ParsedCommand::Plant(crop) => {
            state.selected_seed = crop;
            state.plant_action();

            CommandResult::new(state.message.clone())
                .with_events(vec![format!("Planted {}", crop.seed_name())])
                .with_state_delta(serde_json::json!({
                    "selected_seed": crop.seed_name()
                }))
        }
        ParsedCommand::Water => {
            state.water_action();
            CommandResult::new(state.message.clone()).with_events(vec!["Watered crop".to_string()])
        }
        ParsedCommand::Harvest => {
            state.harvest_action();
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
            if state.in_home() {
                state.close_home();
                state.message = String::from("Good morning! Ready for another day.");
            } else {
                state.home_state = crate::state::HomeState::Alert;
                state.message = String::from("Sleeping... (Income calculated)");
            }

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
        assert!(matches!(result, Ok(ParsedCommand::Clear)));
    }

    #[test]
    fn test_parse_plant() {
        let result = parse_command("plant:carrot");
        assert!(matches!(result, Ok(ParsedCommand::Plant(CropType::Carrot))));
    }

    #[test]
    fn test_parse_water() {
        let result = parse_command("water");
        assert!(matches!(result, Ok(ParsedCommand::Water)));
    }

    #[test]
    fn test_parse_harvest() {
        let result = parse_command("harvest");
        assert!(matches!(result, Ok(ParsedCommand::Harvest)));
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
    fn test_parse_invalid_direction() {
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
        assert!(matches!(result, Ok(ParsedCommand::Plant(CropType::Carrot))));
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
        assert!(matches!(result, Ok(ParsedCommand::Plant(CropType::Carrot))));

        let result = parse_command("plant:strawberry");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Strawberry))
        ));

        let result = parse_command("plant:cauliflower");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Cauliflower))
        ));

        let result = parse_command("plant:rhubarb");
        assert!(matches!(
            result,
            Ok(ParsedCommand::Plant(CropType::Rhubarb))
        ));
    }

    #[test]
    fn test_execute_plant() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Plant(CropType::Carrot));
        assert!(!result.message.is_empty());
        assert!(result.state_delta.is_some());
    }

    #[test]
    fn test_execute_water() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Water);
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_harvest() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Harvest);
        assert!(!result.message.is_empty());
    }

    #[test]
    fn test_execute_clear() {
        let mut state = GameState::new();
        let result = execute_command(&mut state, ParsedCommand::Clear);
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
}
