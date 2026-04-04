mod db;
mod economy;
mod entity;
mod map;
mod state;
mod tile;
mod ui;
mod weather;

use entity::Direction;
use state::GameState;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_help();
            return;
        }
        "-V" | "--version" => {
            println!("tinydew {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        _ => {}
    }

    let conn = db::open_db();
    let mut state = db::load_state(&conn).unwrap_or_else(|| {
        let s = GameState::new();
        db::save_state(&conn, &s);
        s
    });

    match args[1].as_str() {
        "status" => {
            print!("{}", ui::render_status(&state));
        }
        "do" => {
            if args.len() < 3 {
                eprintln!("Usage: tinydew do <ACTION> [ARGS...]");
                std::process::exit(1);
            }

            let action = args[2].as_str();
            match action {
                "move" => {
                    let dir = parse_direction(&args, 3);
                    state.do_move(dir);
                }
                "water" => {
                    let dir = parse_direction(&args, 3);
                    state.do_water(dir);
                }
                "clear" => {
                    let dir = parse_direction(&args, 3);
                    state.do_clear(dir);
                }
                "plant" => {
                    let dir = parse_direction(&args, 3);
                    state.do_plant(dir);
                }
                "harvest" => {
                    let dir = parse_direction(&args, 3);
                    state.do_harvest(dir);
                }
                "buy" => {
                    if args.len() < 4 {
                        eprintln!("Usage: tinydew do buy <ITEM>");
                        std::process::exit(1);
                    }
                    state.do_buy(&args[3]);
                }
                "sell" => {
                    if args.len() < 4 {
                        eprintln!("Usage: tinydew do sell <ITEM_EMOJI>");
                        std::process::exit(1);
                    }
                    state.do_sell(&args[3]);
                }
                "fish" => {
                    let dir = parse_direction(&args, 3);
                    state.do_fish(dir);
                }
                "sleep" => {
                    state.do_sleep();
                }
                _ => {
                    eprintln!("Unknown action: {action}");
                    std::process::exit(1);
                }
            }

            // Print status after action
            print!("{}", ui::render_status(&state));

            // Auto-save
            db::save_state(&conn, &state);
        }
        cmd => {
            eprintln!("Unknown command: {cmd}");
            eprintln!("Run 'tinydew --help' for usage.");
            std::process::exit(1);
        }
    }
}

fn parse_direction(args: &[String], idx: usize) -> Direction {
    if args.len() <= idx {
        eprintln!("Missing direction. Use: up, down, left, right");
        std::process::exit(1);
    }
    args[idx].parse::<Direction>().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    })
}

fn print_help() {
    println!(
        "tinydew {} - A cozy farming game

USAGE:
    tinydew [OPTIONS] <COMMAND> [ARGS...]

OPTIONS:
    -h, --help       Display help information
    -V, --version    Display version information

COMMANDS:
    status           Show the current game status
    do <ACTION>      Execute an action in the game world

ACTIONS:
    move <DIR>       Move character (up, down, left, right)
    water <DIR>      Water a crop in direction
    clear <DIR>      Clear ground in direction
    plant <DIR>      Plant a seed in direction
    harvest <DIR>    Harvest a crop in direction
    buy <ITEM>       Buy items (e.g., seed)
    sell <EMOJI>     Sell items (e.g., 🍓, 🍄)
    fish <DIR>       Fish in direction
    sleep            Sleep through the night",
        env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Direction;
    use crate::map::Location;
    use crate::tile::TileType;

    fn fresh_state() -> GameState {
        // Use a temporary DB for testing
        unsafe {
            std::env::set_var("TINYDEW_DB_PATH", "/tmp/tinydew_test.sqlite");
        }
        let _ = std::fs::remove_file("/tmp/tinydew_test.sqlite");
        GameState::new()
    }

    #[test]
    fn test_initial_state() {
        let state = fresh_state();
        assert_eq!(state.day, 1);
        assert_eq!(state.time_hour, 6);
        assert_eq!(state.time_minute, 0);
        assert_eq!(state.player.x, 3);
        assert_eq!(state.player.y, 3);
        assert_eq!(state.player.location, Location::Farm);
        assert_eq!(state.money, 500);
        assert_eq!(state.inventory.seeds, 5);
    }

    #[test]
    fn test_movement() {
        let mut state = fresh_state();
        state.do_move(Direction::Down);
        assert_eq!(state.player.y, 4);
        assert_eq!(state.time_minute, 5);
    }

    #[test]
    fn test_movement_blocked_by_boundary() {
        let mut state = fresh_state();
        // Move to boundary
        state.player.x = 1;
        state.player.y = 1;
        state.do_move(Direction::Up);
        // Should be blocked (y=0 is boundary)
        assert_eq!(state.player.y, 1);
    }

    #[test]
    fn test_clear_and_plant() {
        let mut state = fresh_state();
        // Position player next to boundary
        state.player.x = 1;
        state.player.y = 1;

        // Clear up (targeting boundary at y=0) should fail
        state.do_clear(Direction::Up);
        assert!(
            state.message.contains("can't"),
            "Expected failure message, got: {}",
            state.message
        );

        // Position to clear a grass tile
        state.player.x = 3;
        state.player.y = 2;
        state.do_clear(Direction::Down);
        let map = state.current_map();
        assert!(matches!(map.get(3, 3), Some(TileType::Soil)));

        // Plant on the soil
        state.do_plant(Direction::Down);
        let map = state.current_map();
        assert!(matches!(map.get(3, 3), Some(TileType::Crop(_))));
        assert_eq!(state.inventory.seeds, 4);
    }

    #[test]
    fn test_buy_seed() {
        let mut state = fresh_state();
        state.do_buy("seed");
        assert_eq!(state.inventory.seeds, 6);
        assert_eq!(state.money, 480);
    }

    #[test]
    fn test_buy_seed_not_enough_money() {
        let mut state = fresh_state();
        state.money = 10;
        state.do_buy("seed");
        assert_eq!(state.inventory.seeds, 5);
        assert_eq!(state.money, 10);
    }

    #[test]
    fn test_region_transition_farm_to_eastpath() {
        let mut state = fresh_state();
        // Position near the east path gate
        state.player.x = 6;
        state.player.y = 5;
        state.do_move(Direction::Right);
        // Should transition to EastPath
        assert_eq!(state.player.location, Location::EastPath);
        assert_eq!(state.player.x, 1);
        assert_eq!(state.player.y, 2);
    }

    #[test]
    fn test_sleep_advances_day() {
        let mut state = fresh_state();
        state.do_sleep();
        assert_eq!(state.day, 2);
        assert_eq!(state.time_hour, 6);
        assert_eq!(state.time_minute, 0);
        assert_eq!(state.player.x, 3);
        assert_eq!(state.player.y, 3);
        assert_eq!(state.player.location, Location::Farm);
    }

    #[test]
    fn test_time_advances() {
        let mut state = fresh_state();
        assert_eq!(state.time_hour, 6);
        assert_eq!(state.time_minute, 0);
        state.do_move(Direction::Down);
        assert_eq!(state.time_minute, 5);
    }

    #[test]
    fn test_weather_day1_sunny() {
        let state = fresh_state();
        assert_eq!(state.weather, crate::weather::Weather::Sunny);
    }

    #[test]
    fn initial_farm_ui() {
        let state = fresh_state();
        let output = crate::ui::render_status(&state);
        // Should contain header
        assert!(output.contains("tinydew day 1"));
        // Should contain player emoji
        assert!(output.contains("🧑"));
        // Should contain money
        assert!(output.contains("Money: 💰 $500"));
        // Should contain seeds
        assert!(output.contains("seeds: 🫙 x5"));
        // Print for visual inspection
        println!("{output}");
    }

    #[test]
    fn test_sell_mushroom() {
        let mut state = fresh_state();
        state.inventory.mushrooms = 1;
        state.do_sell("🍄");
        assert_eq!(state.inventory.mushrooms, 0);
        assert_eq!(state.money, 525); // 500 + 25
    }

    #[test]
    fn test_water_crop() {
        let mut state = fresh_state();
        // Set up a crop
        state.player.x = 3;
        state.player.y = 2;
        state.do_clear(Direction::Down);
        state.do_plant(Direction::Down);

        // Water it
        state.do_water(Direction::Down);
        let map = state.current_map();
        if let Some(TileType::Crop(data)) = map.get(3, 3) {
            assert!(data.watered_today);
        } else {
            panic!("Expected crop tile");
        }
    }

    #[test]
    fn test_cant_plant_on_non_farm() {
        let mut state = fresh_state();
        state.player.location = Location::EastPath;
        state.player.x = 3;
        state.player.y = 1;
        state.do_plant(Direction::Down);
        assert!(state.message.contains("only plant on the Farm"));
    }

    #[test]
    fn test_db_persistence() {
        let db_path = "/tmp/tinydew_persist_test.sqlite";
        unsafe {
            std::env::set_var("TINYDEW_DB_PATH", db_path);
        }
        let _ = std::fs::remove_file(db_path);

        let conn = crate::db::open_db();
        let mut state = GameState::new();
        state.do_move(Direction::Down);
        crate::db::save_state(&conn, &state);

        let loaded = crate::db::load_state(&conn).unwrap();
        assert_eq!(loaded.player.y, 4);
        assert_eq!(loaded.time_minute, 5);

        let _ = std::fs::remove_file(db_path);
    }

    #[test]
    fn test_festival_day28() {
        let mut state = fresh_state();
        // Advance to day 28
        for _ in 1..28 {
            state.do_sleep();
        }
        assert_eq!(state.day, 28);
        assert!(state.message.contains("Butterfly Festival"));

        // Check Wonder tile at Square (2,2)
        let square = state.maps.get("Square").unwrap();
        assert!(matches!(square.tiles[2][2], TileType::Wonder));
    }
}
