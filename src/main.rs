mod state;
mod world;

use crate::state::GameState;
use crate::world::Direction;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};

fn render(game: &GameState) {
    let mut stdout = stdout();

    stdout.execute(crossterm::cursor::SavePosition).unwrap();
    stdout
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();

    print_header(&mut stdout, game);
    print_map(&mut stdout, game);
    print_message(&mut stdout, game);

    stdout.execute(crossterm::cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
}

fn print_header<W: Write>(w: &mut W, game: &GameState) {
    writeln!(
        w,
        "🌸 {} Day {} {} {}",
        game.season,
        game.day,
        game.get_weather_icon(),
        game.format_time()
    )
    .unwrap();
    writeln!(w).unwrap();
}

fn print_map<W: Write>(w: &mut W, game: &GameState) {
    let map = game.get_current_map_ref();
    let (width, height) = game.get_map_size();

    for y in 0..height {
        for x in 0..width {
            let tile = if x == game.player_x && y == game.player_y {
                "🧑‍🌾"
            } else {
                map[y][x].to_emoji()
            };
            write!(w, "{}", tile).unwrap();
        }
        writeln!(w).unwrap();
    }
}

fn print_message<W: Write>(w: &mut W, game: &GameState) {
    writeln!(w).unwrap();
    writeln!(w, "{}", game.message).unwrap();
    writeln!(w).unwrap();
    writeln!(w, "Arrow keys: Move | Esc: Quit").unwrap();
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
