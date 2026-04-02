mod block_key;
mod cli;
mod economy;
mod entity;
mod farming;
mod festival;
mod fishing;
mod grow;
mod map;
mod movement;
mod piano;
mod sleep;
mod spawn;
mod state;
mod time;
mod tui;
mod ui;
mod weather;

use clap::{Parser, Subcommand};
use state::GameState;

#[derive(Parser)]
#[command(name = "tinydew", version, about = "A tiny farming game")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Launch interactive TUI mode
    #[arg(long)]
    #[cfg(feature = "interactive")]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Show game status
    Status,
    /// Execute an action
    Do {
        /// The action to perform
        action: String,
        /// Additional arguments
        args: Vec<String>,
    },
}

fn main() {
    #[cfg(feature = "interactive")]
    {
        // Check for --interactive before clap parsing
        let args: Vec<String> = std::env::args().collect();
        if args.iter().any(|a| a == "--interactive") {
            if let Err(e) = tui::interactive::run() {
                eprintln!("TUI error: {}", e);
            }
            return;
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            let state = GameState::load();
            println!("{}", ui::render_status(&state));
        }
        Commands::Do { action, args } => {
            let mut state = GameState::load();
            let result = cli::dispatch_action(&mut state, &action, &args);
            println!("{}", result);
            state.save();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Direction;
    use crate::map::Region;
    use crate::weather::Weather;

    fn fresh_state() -> GameState {
        // Remove any save file to get clean state
        let _ = std::fs::remove_file("tinydew_save.json");
        GameState::default()
    }

    #[test]
    fn test_initial_state() {
        let state = fresh_state();
        assert_eq!(state.day, 1);
        assert_eq!(state.time.hour, 6);
        assert_eq!(state.time.minute, 0);
        assert_eq!(state.weather, Weather::Sunny);
        assert_eq!(state.player.x, 3);
        assert_eq!(state.player.y, 3);
        assert_eq!(state.player.location, Region::Farm);
        assert_eq!(state.inventory.seeds, 5);
        assert_eq!(state.inventory.money, 100);
    }

    #[test]
    fn test_movement() {
        let mut state = fresh_state();
        let msg = movement::move_player(&mut state, Direction::Right);
        assert_eq!(msg, "Moved right.");
        assert_eq!(state.player.x, 4);
        assert_eq!(state.player.y, 3);
        assert_eq!(state.time.minute, 5); // time ticked
    }

    #[test]
    fn test_movement_blocked_by_tree() {
        let mut state = fresh_state();
        // Move to edge: player at (3,3), move up twice -> (3,1) which is boundary
        state.player.x = 1;
        state.player.y = 1;
        let msg = movement::move_player(&mut state, Direction::Up);
        assert_eq!(msg, "Can't go there.");
        assert_eq!(state.player.y, 1); // didn't move
    }

    #[test]
    fn test_region_transition() {
        let mut state = fresh_state();
        // Move player to gate at Farm (7,5)
        state.player.x = 6;
        state.player.y = 5;
        let msg = movement::move_player(&mut state, Direction::Right);
        assert_eq!(msg, "Entered East Path.");
        assert_eq!(state.player.location, Region::EastPath);
        assert_eq!(state.player.x, 1);
        assert_eq!(state.player.y, 2);
    }

    #[test]
    fn test_buy_sell() {
        let mut state = fresh_state();
        let msg = state.inventory.buy("seed", 2).unwrap();
        assert!(msg.contains("Bought 2"));
        assert_eq!(state.inventory.seeds, 7);
        assert_eq!(state.inventory.money, 80);

        state.inventory.mushrooms = 3;
        let msg = state.inventory.sell("mushroom", 2).unwrap();
        assert!(msg.contains("Sold 2"));
        assert_eq!(state.inventory.mushrooms, 1);
        assert_eq!(state.inventory.money, 130);
    }

    #[test]
    fn test_sleep_advances_day() {
        let mut state = fresh_state();
        let msg = sleep::sleep(&mut state);
        assert_eq!(state.day, 2);
        assert_eq!(state.time.hour, 6);
        assert_eq!(state.time.minute, 0);
        assert_eq!(state.player.x, 3);
        assert_eq!(state.player.y, 3);
        assert_eq!(state.player.location, Region::Farm);
    }

    #[test]
    fn test_weather_day1_sunny() {
        let w = weather::roll_weather(1);
        assert_eq!(w, Weather::Sunny);
        assert_eq!(w.icon(false), "☀️");
        assert_eq!(w.icon(true), "🌙");
    }

    #[test]
    fn test_piano_play_requires_position() {
        let mut state = fresh_state();
        // Player at (3,3) — not at piano position (4,3)
        let msg = cli::dispatch_action(&mut state, "play", &["C4".to_string()]);
        assert_eq!(msg, "Not near the piano.");

        // Move player to (4,3)
        state.player.x = 4;
        state.player.y = 3;
        let msg = cli::dispatch_action(&mut state, "play", &["C4".to_string()]);
        assert_eq!(msg, "🎵 C4");
    }
}
