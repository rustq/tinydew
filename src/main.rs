mod state;
mod world;

use crate::state::GameState;
use crate::world::Direction;
use crossterm::{
    cursor::MoveTo,
    ExecutableCommand,
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
                map[y][x].to_emoji()
            };
            write!(w, "{}", tile).unwrap();
        }
        write!(w, "{}", EOL).unwrap();
    }
}

fn print_message<W: Write>(w: &mut W, game: &GameState) {
    write!(w, "{}", EOL).unwrap();
    write!(w, "{}{}", game.message, EOL).unwrap();
    write!(w, "{}", EOL).unwrap();
    write!(w, "Arrow keys: Move | Esc: Quit{}", EOL).unwrap();
}

fn handle_input(game: &mut GameState) -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
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
                    map[y][x].to_emoji()
                };
                output.push_str(tile);
            }
            output.push('\n');
        }

        output.push('\n');
        output.push_str(&format!("{}\n", game.message));
        output.push('\n');
        output.push_str("Arrow keys: Move | Esc: Quit\n");

        println!("{}", output);
        assert!(output.contains("Spring Day 1"));
        assert!(output.contains("🧑"));
        assert!(output.contains("Welcome to Shelldew!"));
    }
}
