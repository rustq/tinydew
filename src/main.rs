#![allow(clippy::needless_range_loop)]

mod mcp;
mod state;
mod world;

use crate::state::GameState;
use crate::world::Direction;
use crossterm::{
    ExecutableCommand,
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};

fn render(game: &GameState) {
    let mut stdout = stdout();

    stdout
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();

    print_header(&mut stdout, game);
    print_map(&mut stdout, game);
    print_message(&mut stdout, game);

    stdout.flush().unwrap();
}

/// CRLF so the terminal returns to column 0 each line (avoids staircase in raw mode).
const EOL: &str = "\r\n";

fn print_header<W: Write>(w: &mut W, game: &GameState) {
    write!(
        w,
        "🌸 {} Day {} {} {}{}",
        game.season,
        game.day,
        game.get_weather_icon(),
        game.format_time(),
        EOL
    )
    .unwrap();
    write!(w, "{}", EOL).unwrap();
}

fn print_map<W: Write>(w: &mut W, game: &GameState) {
    let map = game.get_current_map_ref();
    let (width, height) = game.get_map_size();

    for y in 0..height {
        for x in 0..width {
            let tile = if x == game.player_x && y == game.player_y {
                "🧑"
            } else {
                map[y][x].emoji()
            };
            write!(w, "{}", tile).unwrap();
        }
        write!(w, "{}", EOL).unwrap();
    }
}

fn print_message<W: Write>(w: &mut W, game: &GameState) {
    write!(w, "{}", EOL).unwrap();
    if game.in_home() {
        print_home_menu(w, game);
    } else if game.in_shop() {
        print_shop_menu(w, game);
    } else {
        write!(w, "{}{}", game.message, EOL).unwrap();
        write!(w, "{}", EOL).unwrap();
        write!(
            w,
            "Arrow keys: Move | C: Clear | P: Plant | W: Water | H: Harvest | T: Trade | Esc: Quit{}",
            EOL
        )
        .unwrap();
    }
}

fn print_shop_menu<W: Write>(w: &mut W, game: &GameState) {
    let title = match game.shop_state {
        state::ShopState::BuyMenu => "Shop",
        state::ShopState::SellMenu => "Sell",
        _ => "Menu",
    };

    write!(w, "💰 ${}{}", game.money, EOL).unwrap();
    write!(w, "{}", EOL).unwrap();
    write!(w, "{}{}", title, EOL).unwrap();

    let items = game.get_shop_menu_items();
    for (i, item) in items.iter().enumerate() {
        let prefix = if i == game.shop_cursor {
            "[√]"
        } else {
            "[ ]"
        };
        write!(w, "{} {}{}", prefix, item, EOL).unwrap();
    }

    write!(w, "{}", EOL).unwrap();
    write!(w, "↑↓: Move | Enter: Confirm | Esc: Back{}", EOL).unwrap();
}

fn print_home_menu<W: Write>(w: &mut W, game: &GameState) {
    match game.home_state {
        state::HomeState::Alert => {
            write!(w, "Home{}", EOL).unwrap();
            write!(w, "{}", EOL).unwrap();

            let items = game.get_home_menu_items();
            for (i, item) in items.iter().enumerate() {
                let prefix = if i == game.home_cursor {
                    "[√]"
                } else {
                    "[ ]"
                };
                write!(w, "{} {}{}", prefix, item, EOL).unwrap();
            }

            write!(w, "{}", EOL).unwrap();
            write!(w, "Enter: Confirm{}", EOL).unwrap();
        }
        state::HomeState::Income => {
            write!(w, "Income this day{}", EOL).unwrap();
            write!(w, "{}", EOL).unwrap();

            if game.current_income.money_earned > 0 {
                write!(w, "💰 * {}{}", game.current_income.money_earned, EOL).unwrap();
            }

            for (crop, count) in &game.current_income.crops_sold {
                if *count > 0 {
                    write!(w, "{} * {}{}", crop.produce_emoji(), count, EOL).unwrap();
                }
            }

            for (forage, count) in &game.current_income.forage_sold {
                if *count > 0 {
                    write!(w, "{} * {}{}", forage.emoji(), count, EOL).unwrap();
                }
            }

            if game.current_income.money_earned == 0
                && game.current_income.crops_sold.is_empty()
                && game.current_income.forage_sold.is_empty()
            {
                write!(w, "(No income today){}", EOL).unwrap();
            }

            write!(w, "{}", EOL).unwrap();

            let items = game.get_home_menu_items();
            for (i, item) in items.iter().enumerate() {
                let prefix = if i == game.home_cursor {
                    "[√]"
                } else {
                    "[ ]"
                };
                write!(w, "{} {}{}", prefix, item, EOL).unwrap();
            }

            write!(w, "{}", EOL).unwrap();
            write!(w, "Enter: Continue{}", EOL).unwrap();
        }
        state::HomeState::None => {}
    }
}

fn handle_input(game: &mut GameState) -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
            if game.in_home() {
                return game.home_handle_input(key.code);
            }

            if game.in_shop() {
                return game.shop_handle_input(key.code);
            }

            match key.code {
                KeyCode::Up => {
                    game.move_player(Direction::Up);
                }
                KeyCode::Down => {
                    game.move_player(Direction::Down);
                }
                KeyCode::Left => {
                    game.move_player(Direction::Left);
                }
                KeyCode::Right => {
                    game.move_player(Direction::Right);
                }
                KeyCode::Esc => {
                    return false;
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    game.clear_action();
                }
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    game.plant_action();
                }
                KeyCode::Char('w') | KeyCode::Char('W') => {
                    game.water_action();
                }
                KeyCode::Char('h') | KeyCode::Char('H') => {
                    game.harvest_action();
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    game.trade_action();
                }
                _ => {}
            }
        }
    }
    true
}

fn main() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();

    let mut game = GameState::new();
    render(&game);

    loop {
        if !handle_input(&mut game) {
            break;
        }
        game.check_home_alert();
        render(&game);
    }

    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_farm_ui() {
        let game = GameState::new();
        let mut output = String::new();

        output.push_str(&format!(
            "{} Day {} {} {}\n\n",
            game.season,
            game.day,
            game.get_weather_icon(),
            game.format_time()
        ));

        let map = game.get_current_map_ref();
        let (width, height) = game.get_map_size();

        for y in 0..height {
            for x in 0..width {
                let tile = if x == game.player_x && y == game.player_y {
                    "🧑"
                } else {
                    map[y][x].emoji()
                };
                output.push_str(tile);
            }
            output.push('\n');
        }

        output.push('\n');
        output.push_str(&format!("{}\n", game.message));
        output.push('\n');
        output.push_str("Arrow keys: Move | C: Clear | P: Plant | W: Water | H: Harvest | T: Trade | Esc: Quit\n");

        println!("{}", output);
        assert!(output.contains("Spring Day 1"));
        assert!(output.contains("🧑"));
        assert!(output.contains("Welcome to Shelldew!"));
    }

    #[test]
    fn home_alert_triggers_at_2am() {
        let mut game = GameState::new();
        game.hour = 1;
        game.minute = 55;
        game.location = state::Location::Farm;

        game.check_home_alert();
        assert_eq!(game.home_state, state::HomeState::None);

        game.hour = 2;
        game.check_home_alert();
        assert_eq!(game.home_state, state::HomeState::Alert);
    }

    #[test]
    fn home_alert_not_in_east_path() {
        let mut game = GameState::new();
        game.hour = 2;
        game.location = state::Location::EastPath;

        game.check_home_alert();
        assert_eq!(game.home_state, state::HomeState::None);
    }

    #[test]
    fn sleep_transitions_to_income() {
        let mut game = GameState::new();
        game.home_state = state::HomeState::Alert;
        game.home_cursor = 0;

        game.home_handle_input(crossterm::event::KeyCode::Enter);
        assert_eq!(game.home_state, state::HomeState::Income);
    }

    #[test]
    fn income_tracks_earnings() {
        let mut game = GameState::new();

        game.record_income(100);
        assert_eq!(game.current_income.money_earned, 100);

        game.record_crop_sold(crate::world::CropType::Carrot, 2);
        assert_eq!(
            game.current_income
                .crops_sold
                .get(&crate::world::CropType::Carrot),
            Some(&2)
        );
    }

    #[test]
    fn close_home_resets_income() {
        let mut game = GameState::new();
        game.current_income.money_earned = 500;

        game.close_home();
        assert_eq!(game.current_income.money_earned, 0);
        assert_eq!(game.home_state, state::HomeState::None);
    }

    #[test]
    fn movement_updating() {
        let mut game = GameState::new();
        assert_eq!(game.player_x, 3);
        assert_eq!(game.player_y, 3);

        game.move_player(crate::world::Direction::Up);
        assert_eq!(game.player_y, 2);

        game.move_player(crate::world::Direction::Down);
        assert_eq!(game.player_y, 3);
    }

    #[test]
    fn movement_blocks_boundaries() {
        let mut game = GameState::new();
        game.player_x = 0;
        game.player_y = 0;

        assert!(!game.can_move_to(0, 0));
    }

    #[test]
    fn clear_grass_to_soil() {
        let mut game = GameState::new();
        game.direction = crate::world::Direction::Right;
        game.player_x = 4;

        game.clear_action();

        let tile = game.get_tile_at(5, 3);
        assert_eq!(tile, Some(crate::world::TileType::Soil));
    }

    #[test]
    fn plant_on_soil() {
        let mut game = GameState::new();
        game.direction = crate::world::Direction::Right;
        game.player_x = 4;
        game.clear_action();

        game.inventory.add_seeds(crate::world::CropType::Carrot, 5);
        game.plant_action();

        let tile = game.get_tile_at(5, 3);
        if let Some(crate::world::TileType::Crop(crop, _)) = tile {
            assert_eq!(crop, crate::world::CropType::Carrot);
        } else {
            panic!("Expected Crop tile");
        }
    }

    #[test]
    fn water_crop() {
        let mut game = GameState::new();
        game.direction = crate::world::Direction::Right;
        game.player_x = 4;
        game.clear_action();
        game.inventory.add_seeds(crate::world::CropType::Carrot, 5);
        game.plant_action();

        game.water_action();

        if let Some(crate::world::TileType::Crop(_, state)) = game.get_tile_at(5, 3) {
            assert!(state.watered_today);
        } else {
            panic!("Expected Crop tile");
        }
    }

    #[test]
    fn harvest_mature_crop() {
        let mut game = GameState::new();
        game.direction = crate::world::Direction::Right;
        game.player_x = 4;
        game.clear_action();
        game.inventory.add_seeds(crate::world::CropType::Carrot, 5);
        game.plant_action();

        for _ in 0..4 {
            game.direction = crate::world::Direction::Right;
            game.player_x = 4;
            game.water_action();
            game.start_new_day();
        }

        let produce_count_before = game.inventory.get_produce(crate::world::CropType::Carrot);
        game.harvest_action();
        let produce_count_after = game.inventory.get_produce(crate::world::CropType::Carrot);

        assert_eq!(produce_count_after, produce_count_before + 1);
    }

    #[test]
    fn crop_growth_requires_water() {
        let mut game = GameState::new();
        game.direction = crate::world::Direction::Right;
        game.player_x = 4;
        game.clear_action();
        game.inventory.add_seeds(crate::world::CropType::Carrot, 5);
        game.plant_action();

        game.start_new_day();

        if let Some(crate::world::TileType::Crop(_, state)) = game.get_tile_at(5, 3) {
            assert_eq!(state.days_grown, 0);
        }

        game.water_action();
        game.start_new_day();

        if let Some(crate::world::TileType::Crop(_, state)) = game.get_tile_at(5, 3) {
            assert_eq!(state.days_grown, 1);
        }
    }

    #[test]
    fn shop_menu_opens() {
        let mut game = GameState::new();
        game.trade_action();
        assert_eq!(game.shop_state, state::ShopState::BuyMenu);
    }

    #[test]
    fn mushroom_spawns_on_east_path() {
        let mut game = GameState::new();
        game.location = state::Location::EastPath;
        game.day = 1;

        let initial_mushrooms = count_mushrooms(&game);

        game.start_new_day();

        let after_mushrooms = count_mushrooms(&game);
        assert!(after_mushrooms >= initial_mushrooms);
    }

    fn count_mushrooms(game: &GameState) -> usize {
        let map = game.get_current_map_ref();
        map.iter()
            .flat_map(|row| row.iter())
            .filter(|&&tile| tile == crate::world::TileType::Mushroom)
            .count()
    }

    #[test]
    fn transition_to_east_path() {
        let mut game = GameState::new();
        game.player_x = 6;
        game.player_y = 5;

        game.move_player(crate::world::Direction::Right);

        assert_eq!(game.location, state::Location::EastPath);
    }

    #[test]
    fn transition_back_to_farm() {
        let mut game = GameState::new();
        game.location = state::Location::EastPath;
        game.player_x = 1;
        game.player_y = 2;

        game.move_player(crate::world::Direction::Left);

        assert_eq!(game.location, state::Location::Farm);
    }

    #[test]
    fn cannot_farm_in_east_path() {
        let mut game = GameState::new();
        game.location = state::Location::EastPath;

        game.clear_action();
        assert!(game.message.contains("Cannot clear here"));

        game.plant_action();
        assert!(game.message.contains("Cannot plant here"));

        game.water_action();
        assert!(game.message.contains("Cannot water here"));
    }
}
