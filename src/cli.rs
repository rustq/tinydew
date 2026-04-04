use crate::state::{load_game, save_game};
use crate::types::Direction;
use crate::ui;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tinydew")]
#[command(version = "0.1.0")]
#[command(about = "A cozy farming and exploration game", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Do {
        action: String,
        arg1: Option<String>,
        arg2: Option<String>,
    },
}

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            let state = load_game();
            println!("{}", ui::render_status(&state));
        }
        Commands::Do { action, arg1, arg2 } => {
            let mut state = load_game();
            let args: Vec<String> = arg1.into_iter().chain(arg2).collect();
            let _message = execute_action(&mut state, &action, &args);
            save_game(&state);
            println!("{}", ui::render_status(&state));
        }
    }

    Ok(())
}

fn execute_action(state: &mut crate::state::GameState, action: &str, args: &[String]) -> String {
    let direction = args.first().and_then(|d| match d.to_lowercase().as_str() {
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        "left" => Some(Direction::Left),
        "right" => Some(Direction::Right),
        _ => None,
    });

    match action.to_lowercase().as_str() {
        "move" => {
            if let Some(dir) = direction {
                state.try_move(dir)
            } else {
                "Invalid direction. Use up, down, left, or right.".to_string()
            }
        }
        "water" => {
            let dir = direction.unwrap_or(state.player.direction);
            state.try_water(dir)
        }
        "plant" => {
            let dir = direction.unwrap_or(state.player.direction);
            state.try_plant(dir)
        }
        "harvest" => {
            let dir = direction.unwrap_or(state.player.direction);
            state.try_harvest(dir)
        }
        "clear" => {
            let dir = direction.unwrap_or(state.player.direction);
            state.try_clear(dir)
        }
        "fish" => {
            let dir = direction.unwrap_or(state.player.direction);
            state.try_fish(dir)
        }
        "buy" => {
            if let Some(item) = args.first() {
                match item.to_lowercase().as_str() {
                    "seed" => state.buy_seed(),
                    _ => format!("Unknown item: {}", item),
                }
            } else {
                "Specify an item to buy.".to_string()
            }
        }
        "sell" => {
            if let Some(item) = args.first() {
                state.sell_item(item)
            } else {
                "Specify an item to sell.".to_string()
            }
        }
        "sleep" => state.sleep(),
        _ => format!("Unknown action: {}", action),
    }
}
